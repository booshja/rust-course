fn main() {
    // same as writing 98000, it's a formatting sugar
    let custom_num = 98_000;

    // 16 bit system 0->9, a->f
    let hex_num = 0xfa;

    // 0b = this is a binary number
    // 101011 is your active binary number
    let bin_num = 0b0010_1011;

    let byte_num = b'A';

    println!("{}", custom_num);
    println!("{}", hex_num);
    println!("{}", bin_num);
    println!("{}", byte_num);
}
