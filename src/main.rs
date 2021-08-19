use object::{Object, ObjectSection};

const MEM_SIZE: usize = 32;

#[derive(Debug)]
struct Instruction {
    name: &'static str,
    opcode: u32,
    type_data: InstTypeData,
    type_name: InstTypeName,
}

impl Instruction {
    fn new() -> Self {
        Instruction {
            name: "default",
            opcode: 0,
            type_data: InstTypeData::Unimplemented,
            type_name: InstTypeName::Unimplemented,
        }
    }
}

#[derive(Debug)]
enum InstTypeName {
    R,
    I,
    S,
    B,
    U,
    J,
    Unimplemented,
}

#[derive(Debug)]
enum InstTypeData {
    R {
        rd: usize,
        funct3: u32,
        rs1: usize,
        rs2: usize,
        funct7: u32,
    },
    I {
        rd: usize,
        funct3: u32,
        rs1: usize,
        imm: u32,
    },
    S {
        imm: u32,
        funct3: u32,
        rs1: usize,
        rs2: usize,
    },
    B {
        imm: u32,
        funct3: u32,
        rs1: usize,
        rs2: usize,
    },
    U {
        rd: usize,
        imm: u32,
    },
    J {
        rd: usize,
        imm: u32,
    },
    Unimplemented,
}

struct Cpu {
    memory: [u32; 1024 * MEM_SIZE],
    registers: [u32; 32],
    pc: u32,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            memory: [0; 1024 * MEM_SIZE],
            registers: [0; 32],
            pc: 0,
        }
    }

    fn load(&mut self, path: &str) {
        let file = std::fs::read(path).unwrap();
        let obj = object::File::parse(&*file).unwrap();
        let text_section =
            obj.section_by_name(".text.init").unwrap().data().unwrap();
        let text_section: Vec<u32> =
            text_section.iter().map(|x| *x as u32).collect();
        let text_section = text_section.as_slice();
        self.memory[..text_section.len()].copy_from_slice(text_section);

        self.pc = 0;
    }

    fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        self.memory[index]
            | self.memory[index + 1] << 8
            | self.memory[index + 2] << 16
            | self.memory[index + 3] << 24
    }

    fn decode(&self, inst: u32) -> Instruction {
        let mut instruction = Instruction::new();
        let opcode = inst & 0b1111111;
        instruction.opcode = opcode;
        match opcode {
            // R Type
            0b0110011 => {
                let rd = ((inst >> 7) & 0b11111) as usize;
                let funct3 = (inst >> 12) & 0b111;
                let rs1 = ((inst >> 15) & 0b11111) as usize;
                let rs2 = ((inst >> 20) & 0b11111) as usize;
                let funct7 = (inst >> 25) & 0b1111111;
                instruction.type_data = InstTypeData::R {
                    rd,
                    funct3,
                    rs1,
                    rs2,
                    funct7,
                };
                instruction.type_name = InstTypeName::R;
            }

            // I Type
            0b0010011 | 0b0000011 | 0b1100111 | 0b1110011 => {
                let rd = ((inst >> 7) & 0b11111) as usize;
                let funct3 = (inst >> 12) & 0b111;
                let rs1 = ((inst >> 15) & 0b11111) as usize;
                let imm = ((inst >> 20) & 0b111111111111) as i32 as u32;
                instruction.type_data = InstTypeData::I {
                    rd,
                    funct3,
                    rs1,
                    imm,
                };
                instruction.type_name = InstTypeName::I;
            }

            // S Type
            0b0100011 => {
                let imm4_0 = (inst >> 7) & 0b11111;
                let imm11_5 = (inst >> 25) & 0b1111111;
                let imm = ((imm11_5 << 5) | imm4_0) as i32 as u32;

                let funct3 = (inst >> 12) & 0b111;
                let rs1 = ((inst >> 15) & 0b11111) as usize;
                let rs2 = ((inst >> 20) & 0b11111) as usize;
                instruction.type_data = InstTypeData::S {
                    imm,
                    funct3,
                    rs1,
                    rs2,
                };
                instruction.type_name = InstTypeName::S;
            }

            // B type
            0b1100011 => {
                let imm11 = (inst >> 7) & 0b1;
                let imm4_1 = (inst >> 8) & 0b1111;
                let imm10_5 = (inst >> 25) & 0b111111;
                let imm12 = (inst >> 31) & 0b1;
                let imm = (imm12 << 12)
                    | (imm11 << 11)
                    | (imm10_5 << 5)
                    | (imm4_1 << 1);
                let imm = imm as i32 as u32;

                let funct3 = (inst >> 12) & 0b111;
                let rs1 = ((inst >> 15) & 0b11111) as usize;
                let rs2 = ((inst >> 20) & 0b11111) as usize;
                instruction.type_data = InstTypeData::B {
                    imm,
                    funct3,
                    rs1,
                    rs2,
                };
                instruction.type_name = InstTypeName::B;
            }

            // J type
            0b1101111 => {
                let rd = ((inst >> 7) & 0b11111) as usize;

                let imm19_12 = (inst >> 12) & 0b11111111;
                let imm11 = (inst >> 20) & 0b1;
                let imm10_1 = (inst >> 21) & 0b1111111111;
                let imm20 = (inst >> 31) & 0b1;
                let imm = (imm20 << 20)
                    | (imm19_12 << 12)
                    | (imm11 << 11)
                    | (imm10_1 << 1);
                let imm = imm as i32 as u32;
                instruction.type_data = InstTypeData::J { rd, imm };
                instruction.type_name = InstTypeName::J;
            }

            // U type
            0b0110111 | 0b0010111 => {
                let rd = ((inst >> 7) & 0b11111) as usize;
                let imm = (inst >> 12) & 0b11111111111111111111;
                let imm = (imm << 12) as i32 as u32;
                instruction.type_data = InstTypeData::U { rd, imm };
                instruction.type_name = InstTypeName::U;
            }

            // Fence
            0b0001111 => instruction.name = "fence",

            _ => {
                println!("unimplemented opcode: {:#09b}", opcode)
            }
        }
        instruction
    }

    fn execute(&mut self, inst: Instruction) {
        match inst.type_name {
            InstTypeName::R => {}
            InstTypeName::B => {}
            InstTypeName::J => {}
            InstTypeName::I => {}
            InstTypeName::S => {}
            InstTypeName::U => {}
            InstTypeName::Unimplemented => {}
        }
    }

    fn run(&mut self) {
        loop {
            let inst = self.fetch();
            let opcode = inst & 0b1111111;
            if (self.pc as usize) >= self.memory.len() || opcode == 0 {
                break;
            }
            let inst: Instruction = self.decode(inst);
            self.execute(inst);
            self.pc = self.pc + 4;
        }
    }
}

fn main() {
    let mut cpu = Cpu::new();
    cpu.load("./tests/rv32ui-p-add");
    cpu.run();
}