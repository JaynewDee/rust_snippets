fn main() {
    println!("Hello, world!\n");

    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);

    println!("true AND false is {}", true && false);

    println!("true OR false is {}", true || false);

    println!("NOT true is {}", !true);

    // Bitwise

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 | 0b0101);

    println!("001 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);

    println!("1 << 5 is {}", 1u32 << 5);

    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability:
    println!("One million can be written as {}", 1_000_000u32);
}
