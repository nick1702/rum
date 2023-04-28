use std::env;
use std::io;

use rum::rumload;

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

fn seg_load(b: u32, c: u32, mem: &mut [Vec<u32>]) -> u32 {
    return mem[b as usize][c as usize];
}

fn seg_store(a: u32, b: u32, c: u32, mem: &mut [Vec<u32>]) {
    mem[a as usize][b as usize] = c;
}

fn add(b: u32, c: u32) -> u32 {
    return b + c;
}

fn mult(b: u32, c: u32) -> u32 {
    return b * c;
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

fn map_seg(c: u32, mem: &mut Vec<Vec<u32>>) -> u32 {
    let new_seg = vec![0; c as usize];
    let unique_bit_pattern = mem.len() as u32;
    mem.push(new_seg);
    return unique_bit_pattern;
}

fn unmap_seg(c: u32, mem: &mut Vec<Vec<u32>>) {
    mem[c as usize] = vec![];
}

fn output_opp(c: u32, stdout: &mut dyn io::Write) {
    if c <= 255 {
        stdout.write(&[c as u8]).unwrap();
    } else {
        println!("Error: C is not a valid ASCII character");
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

fn load_prog(b: u32, c: u32, mem: &mut Vec<Vec<u32>>, _registers: &mut [u32; 8], counter: &mut usize) {
    mem[0] = mem[b as usize].clone();
    *counter = mem[0][c as usize] as usize;
}

fn load_val(a: &mut u32, word: u32) {
    *a = word;
}


fn execute_program(
    program: Vec<u32>,
    mem: &mut Vec<Vec<u32>>,
    _input: &dyn io::Read,
    output: &mut dyn io::Write,
    counter: &mut usize,
) {
    let mut registers: [u32; 8] = [0; 8];
    let stdin = io::stdin();

    while *counter < program.len() {
        let opcode = program[*counter] >> 28;
        let reg_a = ((program[*counter] >> 6) & 7) as usize;
        let reg_b = ((program[*counter] >> 3) & 7) as usize;
        let reg_c = (program[*counter] & 7) as usize;
        *counter += 1;

        match Opcode::from_u32(opcode) {
            Some(Opcode::CMov) => if registers[reg_c] != 0 {registers[reg_a] = cmov(registers[reg_b])},
            Some(Opcode::SegLoad) => registers[reg_a] = seg_load(registers[reg_b], registers[reg_c], mem),
            Some(Opcode::SegStore) => seg_store(registers[reg_a], registers[reg_b], registers[reg_c], mem),
            Some(Opcode::Add) => registers[reg_a] = add(registers[reg_b], registers[reg_c]),
            Some(Opcode::Mult) => registers[reg_a] = mult(registers[reg_b], registers[reg_c]),
            Some(Opcode::Div) => if registers[reg_c] != 0 {registers[reg_a] = div(registers[reg_b], registers[reg_c])},
            Some(Opcode::BitNAND) => registers[reg_a] = bit_nand(registers[reg_b], registers[reg_c]),
            Some(Opcode::Halt) => halt(),
            Some(Opcode::MapSeg) => registers[reg_b] = map_seg(registers[reg_c], mem),
            Some(Opcode::UnmapSeg) => unmap_seg(registers[reg_c], mem),
            Some(Opcode::Output) => output_opp(registers[reg_c], output),
            Some(Opcode::Input) => input_opp(&stdin, &mut registers[reg_c]),
            Some(Opcode::LoadProg) => load_prog(registers[reg_b], registers[reg_c], mem, &mut registers, counter),
            Some(Opcode::LoadVal) => load_val(&mut registers[reg_a], program[*counter - 1]),
            None => panic!("Unknown opcode: {}", opcode),
        }
    }
}


fn main() {
    let input = env::args().nth(1);
    let program_data = rumload::load(input.as_deref());

    // let program = parse_program(&program_data);
    let mut mem: Vec<Vec<u32>> = vec![program_data.clone()];

    let mut counter = 0;
    execute_program(
        program_data,
        &mut mem,
        &io::stdin(),
        &mut io::stdout(),
        &mut counter,
    );
}


