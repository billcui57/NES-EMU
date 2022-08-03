fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & PS_ZERO_MASK == 0);
        assert!(cpu.status & PS_NEGATIVE_MASK == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & PS_ZERO_MASK != 0);
        assert!(cpu.status & PS_NEGATIVE_MASK == 0);
    }
}

const PS_CARRY_MASK: u8 = 0b0000_0001;
const PS_ZERO_MASK: u8 = 0b0000_0010;
const PS_INTDIS_MASK: u8 = 0b0000_0100;
const PS_DECIMAL_MASK: u8 = 0b0000_1000;
const PS_OVERFLOW_MASK: u8 = 0b0100_0000;
const PS_NEGATIVE_MASK: u8 = 0b1000_0000;

pub struct CPU {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.register_a = param;

                    if self.register_a == 0 {
                        self.status = self.status | PS_ZERO_MASK;
                    } else {
                        self.status = self.status & !PS_ZERO_MASK;
                    }

                    if (self.register_a as i8) < 0 {
                        self.status = self.status | PS_NEGATIVE_MASK;
                    } else {
                        self.status = self.status & !PS_NEGATIVE_MASK;
                    }
                }
                0x000 => {
                    return;
                }
                _ => todo!(),
            }
        }
    }
}
