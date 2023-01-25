fn main() {
    let is_it_fun: bool = true;
    // i32 -> signed integer of 32 bits
    // -> signed can hold positive and negative values
    let num: i32 = -10;

    // u8 -> unsigned integer of 8 bits
    // 2^8 - 1 = 255 (max value can store with this type)
    let small_num: u8 = 255;

    // i8 -> signed integer of 8 bits
    // -2^7 = -128 -> 2^7 - 1 = 127
    let small_num_2: i8 = -128;

    // operating system related type
    let sys_num: isize = -10;
    let sys_num_2: usize = 10;
}
