struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus
}

struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

impl CPU {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            let description = format!("0x{}{:x}", if prefixed { "cb" } else { "" }, instruction_byte);
            panic!("Unknown instruction found for: 0x{:x}", description)
        };
        self.pc = next_pc;
    }

    fn execute(&mut self, Instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    
                    _ => { /* TODO: support more targets */ }
                }
            }
            Instruction::ADDHL(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                        self.pc.wrapping_add(1)
                    }
                    _ => { /* TODO: support more targets */ self.pc }
                }
            }
            Instruction::ADC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.adc(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    _ => { /* TODO: support more targets */ self.pc }
                }
            }
            Instruction::SUB(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    _ => { /* TODO: support more targets */ self.pc }
                }
            }
            _ => { /* TODO: more instructions */ }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }

    fn addhl(&mut self, value: u8) -> u16 {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
        
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(self.registers.f.carry).overflowing_add(value);
        
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = value.overflowing_sub(self.registers.a);
        
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = true;
        self.registers.f.carry = did_overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }
}