use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

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

fn parse_program(program_data: &[u8]) -> Vec<u32> {
    let mut program: Vec<u32> = Vec::new();
    let mut program_index = 0;

    while program_index < program_data.len() {
        let word = (program_data[program_index] as u32) << 24
            | (program_data[program_index + 1] as u32) << 16
            | (program_data[program_index + 2] as u32) << 8
            | program_data[program_index + 3] as u32;

        program.push(word);
        program_index += 4;
    }

    program
}


fn CMov(a: &mut u32, b: u32, c: u32) {
    if c != 0 {
        *a = b;
    }
}

fn SegLoad(a: &mut u32, b: u32, c: u32, mem: &mut [Vec<u32>]) {
    *a = mem[b as usize][c as usize];
}

fn SegStore(a: u32, b: u32, c: u32, mem: &mut [Vec<u32>]) {
    mem[b as usize][c as usize] = a;
}

fn Add(a: &mut u32, b: u32, c: u32) {
    *a = b + c;
}

fn Mult(a: &mut u32, b: u32, c: u32) {
    *a = b * c;
}

fn Div(a: &mut u32, b: u32, c: u32) {
    if c != 0 {
        *a = b / c;
    }
}

fn BitNAND(a: &mut u32, b: u32, c: u32) {
    *a = !(b & c);
}

fn Halt() {
    std::process::exit(0);
}

fn MapSeg(b: &mut u32, c: u32, mem: &mut Vec<Vec<u32>>) {
    let new_seg = vec![0; c as usize];
    let unique_bit_pattern = mem.len() as u32;
    mem.push(new_seg);
    *b = unique_bit_pattern;
}

fn UnmapSeg(c: u32, mem: &mut Vec<Vec<u32>>) {
    mem[c as usize] = vec![];
}

fn Output(c: u32, stdout: &mut dyn io::Write) {
    if c <= 255 {
        stdout.write(&[c as u8]).unwrap();
    } else {
        println!("Error: C is not a valid ASCII character");
    }
}

fn Input(stdin: &io::Stdin, c: &mut u32) {
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }
    if let Some(ch) = input.chars().next() {
        *c = ch as u32;
    }
}

fn LoadProg(b: u32, c: u32, mem: &mut Vec<Vec<u32>>, registers: &mut [u32; 8], counter: &mut usize) {
    let seg = mem[b as usize].clone();
    *counter = c as usize;
}

fn LoadVal(a: &mut u32, word: u32) {
    *a = word;
}
fn execute_program(
    program: Vec<u32>,
    mem: &mut Vec<Vec<u32>>,
    input: &dyn io::Read,
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
            Some(Opcode::CMov) => CMov(&mut registers[reg_a], registers[reg_b], registers[reg_c]),
            Some(Opcode::SegLoad) => SegLoad(&mut registers[reg_a], registers[reg_b], registers[reg_c], mem),
            Some(Opcode::SegStore) => SegStore(registers[reg_a], registers[reg_b], registers[reg_c], mem),
            Some(Opcode::Add) => Add(&mut registers[reg_a], registers[reg_b], registers[reg_c]),
            Some(Opcode::Mult) => Mult(&mut registers[reg_a], registers[reg_b], registers[reg_c]),
            Some(Opcode::Div) => Div(&mut registers[reg_a], registers[reg_b], registers[reg_c]),
            Some(Opcode::BitNAND) => BitNAND(&mut registers[reg_a], registers[reg_b], registers[reg_c]),
            Some(Opcode::Halt) => Halt(),
            Some(Opcode::MapSeg) => MapSeg(&mut registers[reg_b], registers[reg_c], mem),
            Some(Opcode::UnmapSeg) => UnmapSeg(registers[reg_c], mem),
            Some(Opcode::Output) => Output(registers[reg_c], output),
            Some(Opcode::Input) => Input(&stdin, &mut registers[reg_c]),
            Some(Opcode::LoadProg) => LoadProg(registers[reg_b], registers[reg_c], mem, &mut registers, counter),
            Some(Opcode::LoadVal) => LoadVal(&mut registers[reg_a], program[*counter - 1]),
            None => panic!("Unknown opcode: {}", opcode),
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        return;
    }

    let input_file = &args[1];
    let mut file = File::open(input_file).expect("Unable to open file");
    let mut program_data = Vec::new();
    file.read_to_end(&mut program_data).expect("Unable to read file");

    let program = parse_program(&program_data);
    let mut mem: Vec<Vec<u32>> = vec![program.clone()];

    let mut counter = 0;
    execute_program(
        program,
        &mut mem,
        &io::stdin(),
        &mut io::stdout(),
        &mut counter,
    );
}


