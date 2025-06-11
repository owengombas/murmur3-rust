mod murmur3;

fn main() {
    let data: &[u8] = &[1, 2, 3, 4, 5, 6];
    let hash = murmur3::murmur3_x86_32(data, 0);
    println!("{}", hash);

    let data: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2];
    let hash = murmur3::murmur3_x86_128(data, 0);
    println!("{:?}", hash);

    let data: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1, 2];
    let hash = murmur3::murmur3_x64_128(data, 0);
    println!("{:?}", hash);
}
