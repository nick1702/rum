use std::env;
use std::io;
use num::pow;

use rum::rumload;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug)]
pub struct Segment {
    pub memory: Vec<u32>,
}
#[derive(Debug)]
pub struct SegmentManager {
    segments: HashMap<u32, Segment>,
    unmapped_ids: Vec<u32>,
    next_id: AtomicU32,
}

impl SegmentManager {
    pub fn new() -> Self {
        SegmentManager {
            segments: HashMap::new(),
            unmapped_ids: Vec::new(),
            next_id: AtomicU32::new(0),
        }
    }

    pub fn allocate_segment(&mut self, size: usize) -> u32 {
        let id = if let Some(reused_id) = self.unmapped_ids.pop() {
            reused_id
        } else {
            let next_id = self.next_id.fetch_add(1, Ordering::SeqCst);
            next_id
        };
        self.segments.insert(id, Segment { memory: vec![0; size] });
        id
    }

    pub fn deallocate_segment(&mut self, id: u32) {
        if self.segments.remove(&id).is_some() {
            self.unmapped_ids.push(id);
        }
    }

    pub fn get_segment_mut(&mut self, id: u32) -> Option<&mut Segment> {
        self.segments.get_mut(&id)
    }
}



enum Opcode {
  CMov = 0,
  SegLoad = 1,
  SegStore = 2,
  Add = 3,
  Mult = 4,
  Div = 5,
  BitNAND = 6,
  Halt = 7,
  MapSeg = 8,
  UnmapSeg = 9,
  Output = 10,
  Input = 11,
  LoadProg = 12,
  LoadVal = 13,
}

impl Opcode {
  fn from_u32(value: u32) -> Option<Self> {
      match value {
          0 => Some(Opcode::CMov),
          1 => Some(Opcode::SegLoad),
          2 => Some(Opcode::SegStore),
          3 => Some(Opcode::Add),
          4 => Some(Opcode::Mult),
          5 => Some(Opcode::Div),
          6 => Some(Opcode::BitNAND),
          7 => Some(Opcode::Halt),
          8 => Some(Opcode::MapSeg),
          9 => Some(Opcode::UnmapSeg),
          10 => Some(Opcode::Output),
          11 => Some(Opcode::Input),
          12 => Some(Opcode::LoadProg),
          13 => Some(Opcode::LoadVal),
          _ => None,
      }
  }
}


fn cmov(b: u32) -> u32 {
    return b;
}

fn seg_load(b: u32, c: u32, segment_manager: &mut SegmentManager) -> u32 {
    if let Some(segment) = segment_manager.get_segment_mut(b) {
        if c as usize >= segment.memory.len() {
            panic!("Error: Segmented load out of bounds");
        }
        segment.memory[c as usize]
    } else {
        panic!("Error: Segmented load with unmapped segment");
    }
}

fn seg_store(a: u32, b: u32, c: u32, segment_manager: &mut SegmentManager) {
    if let Some(segment) = segment_manager.get_segment_mut(a) {
        if b as usize >= segment.memory.len() {
            panic!("Error: Segmented store out of bounds");
        }
        segment.memory[b as usize] = c;
    } else {
        panic!("Error: Segmented store with unmapped segment");
    }
}


fn add(b: u32, c: u32) -> u32 {
    return b.wrapping_add(c);
}

fn mult(b: u32, c: u32) -> u32 {
    return b.wrapping_mul(c);
}

fn div(b: u32, c: u32) -> u32 {
    return b / c;
}

fn bit_nand(b: u32, c: u32) -> u32 {
    return !(b & c);
}

fn halt() {
    std::process::exit(0);
}

fn map_seg(c: u32, segment_manager: &mut SegmentManager) -> u32 {
    segment_manager.allocate_segment(c as usize)
}

fn unmap_seg(c: u32, segment_manager: &mut SegmentManager) {
    if c == 0 {
        panic!("Error: Attempt to unmap $m[0]");
    }
    if !segment_manager.segments.contains_key(&c) {
        panic!("Error: Attempt to unmap an unmapped segment");
    }
    segment_manager.deallocate_segment(c);
}

fn output_opp(c: u32, stdout: &mut dyn io::Write) {
    if c <= 255 {
        stdout.write(&[c as u8]).unwrap();
    } else {
        println!("Error: {} is not a valid ASCII character", c);
    }
}

fn input_opp(stdin: &io::Stdin, c: &mut u32) {
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
    if let Some(ch) = input.chars().next() {
        let num = ch as u32;
        if num <= 255 {
            *c = num;
        }
        else{
            *c = 4294967295     //Is this right?
        }
    }
}


fn load_prog(
    b: u32,
    c: u32,
    segment_manager: &mut SegmentManager,
    _registers: &mut [u32; 8],
    counter: &mut usize,
) {
    if let Some(source_segment) = segment_manager.get_segment_mut(b) {
        let source_segment_memory = source_segment.memory.clone();

        if let Some(zero_segment) = segment_manager.get_segment_mut(0) {
            zero_segment.memory = source_segment_memory;
        }

        *counter = c as usize;
    } else {
        // Handle the error case when the segment is not mapped
        eprintln!("Error: Attempt to load program from an unmapped segment");
        std::process::exit(1);
    }
}




fn execute_program(
    segment_manager: &mut SegmentManager,
    _input: &dyn io::Read,
    output: &mut dyn io::Write,
    counter: &mut usize,
) {

    let mut registers: [u32; 8] = [0; 8];
    let stdin = io::stdin();

    loop {
        // println!("current word is {:#b}", segment_manager.get_segment_mut(0).unwrap().memory[*counter]);
        let opcode = segment_manager.get_segment_mut(0).unwrap().memory[*counter] >> 28;
        if opcode == 13{
            registers[((segment_manager.get_segment_mut(0).unwrap().memory[*counter] << 4) >> 29) as usize] = (segment_manager.get_segment_mut(0).unwrap().memory[*counter] << 7) >> 7;
            *counter += 1;
        } else {
            let reg_a = ((segment_manager.get_segment_mut(0).unwrap().memory[*counter] >> 6) & 7) as usize;
            let reg_b = ((segment_manager.get_segment_mut(0).unwrap().memory[*counter] >> 3) & 7) as usize;
            let reg_c = (segment_manager.get_segment_mut(0).unwrap().memory[*counter] & 7) as usize;
            // println!("opcode: {:#b}, reg_a: {:#b}, reg_b: {:#b}, reg_c: {:#b}", opcode, reg_a, reg_b, reg_c);

            *counter += 1;

            match Opcode::from_u32(opcode) {
                Some(Opcode::CMov) => if registers[reg_c] != 0 {registers[reg_a] = cmov(registers[reg_b])},
                Some(Opcode::SegLoad) => registers[reg_a] = seg_load(registers[reg_b], registers[reg_c], segment_manager),
                Some(Opcode::SegStore) => seg_store(registers[reg_a], registers[reg_b], registers[reg_c], segment_manager),
                Some(Opcode::Add) => registers[reg_a] = add(registers[reg_b], registers[reg_c]),
                Some(Opcode::Mult) => registers[reg_a] = mult(registers[reg_b], registers[reg_c]),
                Some(Opcode::Div) => if registers[reg_c] != 0 {registers[reg_a] = div(registers[reg_b], registers[reg_c])},
                Some(Opcode::BitNAND) => registers[reg_a] = bit_nand(registers[reg_b], registers[reg_c]),
                Some(Opcode::Halt) => halt(),
                Some(Opcode::MapSeg) => registers[reg_b] = map_seg(registers[reg_c], segment_manager),
                Some(Opcode::UnmapSeg) => unmap_seg(registers[reg_c], segment_manager),
                Some(Opcode::Output) => output_opp(registers[reg_c], output),
                Some(Opcode::Input) => input_opp(&stdin, &mut registers[reg_c]),
                Some(Opcode::LoadProg) => load_prog(registers[reg_b], registers[reg_c], segment_manager, &mut registers, counter),
                Some(Opcode::LoadVal) => (),
                None => panic!("Unknown opcode: {}", opcode),
            }
        }
    }
}


fn main() {
    let input = env::args().nth(1);
    let program_data = rumload::load(input.as_deref());

    // Initialize the SegmentManager and create the initial segment for the program
    let mut segment_manager = SegmentManager::new();
    let program_id = segment_manager.allocate_segment(program_data.len());
    if let Some(program_segment) = segment_manager.get_segment_mut(program_id) {
        program_segment.memory = program_data.clone();
    }
    let mut counter = 0;
    execute_program(
        &mut segment_manager,
        &io::stdin(),
        &mut io::stdout(),
        &mut counter,
    );
}



