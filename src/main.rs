use std::{env, mem};
use std::io::{self, Read};

use rum::rumload;

use rum::memory::*;



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


/// Copies the value in `b` and returns it.
///
/// # Arguments
///
/// * `b` - A u32 integer containing the value to be copied
#[inline]
fn cmov(b: u32) -> u32 {
    return b;
}

/// Loads the value in the memory cell `c` of the segment `b` and returns it.
/// If the segment is not mapped or `c` is out of bounds, it panics with an error message.
///
/// # Arguments
///
/// * `b` - A u32 integer representing the segment ID
/// * `c` - A u32 integer representing the memory cell index
/// * `segment_manager` - A mutable reference to a SegmentManager instance
#[inline]
fn seg_load(a: u32, b: u32, segment_manager: &mut SegmentManager) -> u32 {
    if a as usize >= segment_manager.segments.len() {
        panic!("Error: Segmented load with unmapped segment");
    }

    let segment = &segment_manager.segments[a as usize];
    if b as usize >= segment.len() {
        panic!("Error: Segmented load out of bounds");
    }

    segment[b as usize]
}
/// Stores the value `c` in the memory cell `b` of the segment `a`.
/// If the segment is not mapped or `b` is out of bounds, it panics with an error message.
///
/// # Arguments
///
/// * `a` - A u32 integer representing the segment ID
/// * `b` - A u32 integer representing the memory cell index
/// * `c` - A u32 integer containing the value to store
/// * `segment_manager` - A mutable reference to a SegmentManager instance
///
#[inline]
fn seg_store(a: u32, b: u32, c: u32, segment_manager: &mut SegmentManager) {
    if let Some(segment) = segment_manager.get_segment_mut(a) {
        if b as usize >= segment.len() {
            panic!("Error: Segmented store out of bounds");
        }
        segment[b as usize] = c;
    } else {
        panic!("Error: Segmented store with unmapped segment");
    }
}

/// Adds `b` and `c` and returns the result.
///
/// # Arguments
///
/// * `b` - A u32 integer
/// * `c` - A u32 integer
#[inline]
fn add(b: u32, c: u32) -> u32 {
    return b.wrapping_add(c);
}

/// Multiplies `b` and `c` and returns the result.
///
/// # Arguments
///
/// * `b` - A u32 integer
/// * `c` - A u32 integer
#[inline]
fn mult(b: u32, c: u32) -> u32 {
    return b.wrapping_mul(c);
}

/// Divides `b` by `c` and returns the result.
///
/// # Arguments
///
/// * `b` - A u32 integer
/// * `c` - A u32 integer
#[inline]
fn div(b: u32, c: u32) -> u32 {
    return b / c;
}

/// Performs a bitwise NAND operation on `b` and `c` and returns the result.
///
/// # Arguments
///
/// * `b` - A u32 integer
/// * `c` - A u32 integer
#[inline]
fn bit_nand(b: u32, c: u32) -> u32 {
    return !(b & c);
}

/// This function terminates the program.
#[inline]
fn halt() {
    std::process::exit(0);
}

/// Creates a new segment with `c` memory cells and returns its ID.
///
/// # Arguments
///
/// * `c` - A u32 integer representing the number of memory cells
/// * `segment_manager` - A mutable reference to a SegmentManager instance
#[inline]
fn map_seg(c: u32, segment_manager: &mut SegmentManager) -> u32 {
    segment_manager.allocate_segment(c as usize)
}

/// Deallocates the segment with ID `c`.
/// If `c` is 0 or the segment is not mapped, it panics with an error message.
///
/// # Arguments
///
/// * `c` - A u32 integer representing the segment ID
/// * `segment_manager` - A mutable reference to a SegmentManager instance
fn unmap_seg(c: u32, segment_manager: &mut SegmentManager) {
    if c == 0 {
        panic!("Error: Attempt to unmap $m[0]");
    }
    if c >= segment_manager.segments.len() as u32 {
    panic!("Error: Attempt to unmap an invalid segment");
    }
    segment_manager.deallocate_segment(c);
}



/// Writes the value `c` to the standard output stream.
/// If `c` is not a valid ASCII character, it prints an error message to the console.
///
/// # Arguments
///
/// * `c` - A u32 integer representing the ASCII value of the character to output
/// * `stdout` - A mutable reference to a dyn io::Write implementation (typically stdout)
fn output_opp(c: u32, stdout: &mut dyn io::Write) {
    if c <= 255 {
        stdout.write(&[c as u8]).unwrap();
    } else {
        println!("Error: {} is not a valid ASCII character", c);
    }
}

/// Reads a single byte from the standard input stream and stores it in `c`.
/// If the input stream is empty, it stores `u32::MAX` in `c`.
/// If an error occurs, it prints an error message to the console and stores `u32::MAX` in `c`. 
///
/// # Arguments
///
/// * `input_iter` - A mutable reference to a std::io::Bytes instance created from StdinLock
/// * `c` - A mutable reference to a u32 integer where the input value will be stored
#[inline]
fn input_opp(input_iter: &mut std::io::Bytes<std::io::StdinLock<'_>>, c: &mut u32) {
    match input_iter.next() {
        Some(Ok(byte)) => {
            *c = byte as u32;
        }
        Some(Err(e)) => {
            eprintln!("Error reading input: {}", e);
            *c = u32::MAX;
        }
        None => {
            *c = u32::MAX;
        }
    }
}

/// Loads the program stored in segment `b` into segment 0 and sets the counter to `c`.
/// If the segment is not mapped, it panics with an error message.
/// Essentially a jump command.
///
/// # Arguments
///
/// * `b` - A u32 integer representing the segment ID containing the program to load
/// * `c` - A u32 integer representing the new program counter value
/// * `segment_manager` - A mutable reference to a SegmentManager instance
/// * `_registers` - A mutable reference to an array of 8 u32 registers (currently unused)
/// * `counter` - A mutable reference to a usize representing the program counter
fn load_prog(
    b: u32,
    c: u32,
    segment_manager: &mut SegmentManager,
    _registers: &mut [u32; 8],
    counter: &mut usize,
) {
    if let Some(source_segment) = segment_manager.get_segment_mut(b) {
        let mut source_segment_memory = source_segment.clone();

        if let Some(zero_segment) = segment_manager.get_segment_mut(0) {
            mem::swap(zero_segment, &mut source_segment_memory);
        }

        *counter = c as usize;
    } else {
        // Handle the error case when the segment is not mapped
        eprintln!("Error: Attempt to load program from an unmapped segment");
        std::process::exit(1);
    }
}


/// Executes the given UM program, interacting with standard input and output as necessary.
/// # Arguments
///
/// * `segment_manager` - A mutable reference to the `SegmentManager` managing the memory segments of the program.
/// * `_input` - A reference to the standard input stream.
/// * `output` - A mutable reference to the standard output stream.
/// * `counter` - A mutable reference to the program counter.
fn execute_program(
    segment_manager: &mut SegmentManager,
    _input: &dyn io::Read,
    output: &mut dyn io::Write,
    counter: &mut usize,
) {
    let mut registers: [u32; 8] = [0; 8];
    let stdin = io::stdin();
    let mut input_iter = stdin.lock().bytes();

    loop {
        let word = segment_manager.get_segment_mut(0).unwrap()[*counter];
        let opcode = word >> 28;
        if opcode == 13 {
            registers[((word << 4) >> 29) as usize] = (word << 7) >> 7;
            *counter += 1;
        } else {
            let reg_a = ((word >> 6) & 7) as usize;
            let reg_b = ((word >> 3) & 7) as usize;
            let reg_c = (word & 7) as usize;

            *counter += 1;

            match Opcode::from_u32(opcode) {
                Some(Opcode::CMov) => {
                    if registers[reg_c] != 0 {
                        registers[reg_a] = cmov(registers[reg_b])
                    }
                }
                Some(Opcode::SegLoad) => registers[reg_a] = seg_load(registers[reg_b], registers[reg_c], segment_manager),
                Some(Opcode::SegStore) => seg_store(registers[reg_a], registers[reg_b], registers[reg_c], segment_manager),
                Some(Opcode::Add) => registers[reg_a] = add(registers[reg_b], registers[reg_c]),
                Some(Opcode::Mult) => registers[reg_a] = mult(registers[reg_b], registers[reg_c]),
                Some(Opcode::Div) => {
                    if registers[reg_c] != 0 {
                        registers[reg_a] = div(registers[reg_b], registers[reg_c])
                    }
                }
                Some(Opcode::BitNAND) => registers[reg_a] = bit_nand(registers[reg_b], registers[reg_c]),
                Some(Opcode::Halt) => halt(),
                Some(Opcode::MapSeg) =>registers[reg_b] = map_seg(registers[reg_c], segment_manager),
                Some(Opcode::UnmapSeg) => unmap_seg(registers[reg_c], segment_manager),
                Some(Opcode::Output) => output_opp(registers[reg_c], output),
                Some(Opcode::Input) => input_opp(&mut input_iter, &mut registers[reg_c]),
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
    // m[0][0] = program_data
    let mut segment_manager = SegmentManager::new();
    let program_id = segment_manager.allocate_segment(program_data.len());
    if let Some(program_segment) = segment_manager.get_segment_mut(program_id) {
        *program_segment = program_data.clone();
    }
    let mut counter = 0;
    execute_program(
        &mut segment_manager,
        &io::stdin(),
        &mut io::stdout(),
        &mut counter,
    );
}



