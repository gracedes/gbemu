enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

impl Instruction {
    fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(PrefixTarget::B)),
            _ => /* TODO: add mapping for rest of instructions*/ None
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            _ => /* TODO: Add mapping for rest of instructions */ None
        }
    }
}