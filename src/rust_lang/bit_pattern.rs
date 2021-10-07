mod test {
    #[test]
    fn test_bit_pattern() {
        let a: u16 = 50115; // unsigned int16
        let b: i16 = -15421; // int16 or signed int16
                             // These two values have the same bit pattern but different types.
        println!("a: {:016b} {}", a, a); // 016: left-pad with 16 zeros, b: binary
        println!("b: {:016b} {}", b, b);

        let a: f32 = 42.42;
        let yum: u32 = unsafe { std::mem::transmute(a) }; // Reinterprets the bits of a value of one type as another type.

        println!("{}", yum);
        println!("{:032b}", yum); // 032: left-pad with 32 zeros, b: binary

        let b: f32 = unsafe { std::mem::transmute(yum) };
        println!("{}", b);
        assert_eq!(a, b);

        let zero: u16 = 0b0000_0000_0000_0000;
        println!("{}", zero);

        // let (a, b) = (200, 200);
        // #[allow(arithmetic_overflow)]
        // let c: u8 = a + b; // error: attempt to compute `200_u8 + 200_u8`, which would overflow
        // println!("200 + 200 = {}", c);
    }

    #[test]
    // CPU vendors argue about how the individual bytes that make up integers should be
    // laid out. Some CPUs order multibyte sequences left to right and others are right to
    // left. This characteristic is known as a CPU’s endianness
    fn test_endian() {
        // 1 vs 16777216
        // let big_endian: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
        // let little_endian: [u8; 4] = [0x00, 0x00, 0x00, 0x001];
        let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
        let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];
        let a: i32 = unsafe { std::mem::transmute(big_endian) };
        let b: i32 = unsafe { std::mem::transmute(little_endian) };
        println!("{} vs {}", a, b);
        // Most computers that people run for day-to-day work print the following: -573785174 vs. -1430532899
        // But more exotic hardware swaps the two numbers around like this: -1430532899 vs. -573785174
    }
}
