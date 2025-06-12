use murmurs::*;

fn main() {
    let data = [1, 2, 3];
    println!("{}", murmur3_x86_32(&data, 0));
    println!("{:?}", murmur3_x86_128(&data, 0));
    println!("{:?}", murmur3_x64_128(&data, 0));
}
