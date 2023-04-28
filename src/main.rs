 use std::fs::File;

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
  
  
  
  // Opcode: 0
  fn CMov(A: mut &u32, B: &u32, C: &u32){
    // if C != 0, then A = B
  }

  // Opcode: 1
  fn SegLoad(A: mut &u32, B: &u32, C: &u32){
    // A = mem[B][C]
  }

  // Opcode: 2
  fn SegStore(A: &u32, B: &u32, C: &u32){
    // mem[B][C] = A
  }

  // Opcode: 3
  fn Add(A: mut &u32, B: &u32, C: &u32){
    // A = B + C
  }

  // Opcode: 4
  fn Mult(A: mut &u32, B: &u32, C: &u32){
    // A = B * C
  }

  // Opcode: 5
  fn Div(A: mut &u32, B: &u32, C: &u32){
    // A = B / C
  }

  // Opcode: 6
  fn BitNAND(A: mut &u32, B: &u32, C: &u32){
    // A = ~(B & C)
  }

  // Opcode: 7
  fn Halt(None){
    // Causes the program to terminate
  }

  // Opcode: 8
  fn MapSeg(B: mut &u32, C: &u32){
    // new_seg = newu(C)
    // for i in 0..C
    //      new_seg[i] = 0
    // unique_bit_pattern = get_unique_bit_pattern()
    // bitpattern_hash_table.insert(unique_bit_pattern)
    // B = unique_bit_pattern
    // mem[B] = new_seg
  }

  // Opcode: 9
  fn UnmapSeg(C: &u32){
    // bitpattern_hash_table.remove(C)
    // mem[C] = None
  }

  // Opcode: 10
  fn Output(C: &u32){
    // if(C >= 0 && C <= 255)
    //   print(C)
    // else
    //   print("Error: C is not a valid ASCII character")
  }

  // Opcode: 11
  fn Input(stdin, C: mut &u32){
    // if(input >= 0 && input <= 255)
    //  C = input
  }

  // Opcode: 12
  fn LoadProg(B: &u32, C: &u32){
    // m[0] = m[B]
    // counter = m[0][C]
  }

  // Opcode: 13
  fn LoadVal(A: mut &u32, word: &u32){
    // A = word
  }

  // Reads Through Vec of u32s and executes the instructions
  fn execute_program(program: Vec<u32>){
    let mut counter = 0;
    let mut registers:Vec<mut u32> = vec![0; 8];

    while counter < program.len() {
      let instruction = program[counter];
      let opcode = instruction >> 28;
      let reg_a = (instruction >> 6) & 0x7;
      let reg_b = (instruction >> 3) & 0x7;
      let reg_c = instruction & 0x7;
      let value = instruction & 0x1FFFFFF;

      match opcode {
        0 => CMov(&mut registers[reg_a], &mut registers[reg_b], &mut registers[reg_c]),
        1 => SegLoad(&mut registers[reg_a], &mut registers[reg_b], &mut registers[reg_c]),
        2 => SegStore(&mut registers[reg_a], &mut registers[reg_b], &mut registers[reg_c]),
        3 => Add(&mut registers[reg_a], &mut registers[reg_b], &mut registers[reg_c]),
        4 => Mult(&mut registers[reg_a], &mut registers[reg_b], &mut registers[reg_c]),
        5 => Div(&mut registers[reg_a], &mut registers[reg_b], &mut registers[reg_c]),
        6 => BitNAND(&mut registers[reg_a], &mut registers[reg_b], &mut registers[reg_c]),
        7 => Halt(),
        8 => MapSeg(&mut registers[reg_b], &mut registers[reg_c]),
        9 => UnmapSeg(&mut registers[reg_c]),
        10 => Output(&mut registers[reg_c]),
        11 => Input(&mut registers[reg_c]),
        12 => LoadProg(&mut registers[reg_b], &mut registers[reg_c]),
        13 => LoadVal(&mut registers[reg_a], value),
        _ => println!("Error: Invalid Opcode")
      }
      counter += 1;
    }
  }

    fn main() {
        let mut program: Vec<u32> = Vec::new();
        let mut input = String::new();
        let mut file = File::open("challenge.bin").expect("Error: File not found");
        file.read_to_string(&mut input).expect("Error: Unable to read file");
        let mut words = input.split_whitespace();
        for word in words {
        let word = u32::from_str_radix(word, 2).unwrap();
        program.push(word);
        }
        execute_program(program);
    }