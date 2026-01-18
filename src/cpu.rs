impl CPU {
    fn execute(&mut self, instructon: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
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
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.addhl(value);
                        self.registers.set_hl(new_value);
                    }
                }
            }

            Instructon::ADC(target) => {
                    match target {
                        ArithmeticTarget::A => {
                            let value = self.registers.a;
                            let new_value = self.adc(value);
                            self.registers.a = new_value;
                        }
                        ArithmeticTarget::B => {
                            let value = self.registers.b;
                            let new_value = self.adc(value);
                            self.registers.a = new_value;
                        }
                        ArithmeticTarget::C => {
                            let value = self.registers.c;
                            let new_value = self.adc(value);
                            self.registers.a = new_value;
                        }
                        ArithmeticTarget::D => {
                            let value = self.registers.d;
                            let new_value = self.adc(value);
                            self.registers.a = new_value;
                        }
                        ArithmeticTarget::E => {
                            let value = self.registers.e;
                            let new_value = self.adc(value);
                            self.registers.a = new_value;
                        }
                        ArithmeticTarget::H => {
                            let value = self.registers.h;
                            let new_value = self.adc(value);
                            self.registers.a = new_value;
                        }
                        ArithmeticTarget::L => {
                            let value = self.registers.l;
                            let new_value = self.adc(value);
                            self.registers.a = new_value;
                        }
                    }
                }

            Instruction::SUB(target) => {
                match target {
                    ArithmeticTarget::A => {
                        let value = self.registers.a;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::B => {
                        let value = self.registers.b;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::D => {
                        let value = self.registers.d;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::E => {
                        let value = self.registers.e;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::H => {
                        let value = self.registers.h;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
                    ArithmeticTarget::L => {
                        let value = self.registers.l;
                        let new_value = self.sub(value);
                        self.registers.a = new_value;
                    }
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