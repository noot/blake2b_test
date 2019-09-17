use blake2_rfc;

fn main() {
    let hash = blake2_rfc::blake2b::blake2b(32, &[], b"");
    println!("blake2b_256 of empty: {:x?}", hash.as_bytes());

    let hash128 = blake2_rfc::blake2b::blake2b(16, &[], b"");
    println!("blake2b_128 of empty: {:x?}", hash128.as_bytes());

    // expected output:
    // blake2b_256 of empty: [e, 57, 51, c0, 26, e5, 43, b2, e8, ab, 2e, b0, 60, 99, da, a1, d1, e5, df, 47, 77, 8f, 77, 87, fa, ab, 45, cd, f1, 2f, e3, a8]
    // blake2b_128 of empty: [ca, e6, 69, 41, d9, ef, bd, 40, 4e, 4d, 88, 75, 8e, a6, 76, 70]

}
