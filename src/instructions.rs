enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}