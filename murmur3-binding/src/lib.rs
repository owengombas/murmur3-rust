#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

mod bindings;

pub fn murmur3_x64_128(key: &[u8], seed: u32) -> [u64; 2] {
    unsafe {
        let output: [u64; 2] = [0; 2];
        bindings::murmur3_x64_128(
            key.as_ptr() as *const _,
            key.len() as i32,
            seed,
            output.as_ptr() as *mut std::ffi::c_void,
        );
        output
    }
}

pub fn murmur3_x86_128(key: &[u8], seed: u32) -> [u32; 4] {
    unsafe {
        let output: [u32; 4] = [0; 4];
        bindings::murmur3_x86_128(
            key.as_ptr() as *const _,
            key.len() as i32,
            seed,
            output.as_ptr() as *mut std::ffi::c_void,
        );
        output
    }
}

pub fn murmur3_x86_32(key: &[u8], seed: u32) -> u32 {
    unsafe {
        let mut output: u32 = 0;
        bindings::murmur3_x86_32(
            key.as_ptr() as *const _,
            key.len() as i32,
            seed,
            &mut output as *mut _ as *mut std::ffi::c_void,
        );
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test = b"hello";
        println!("{:?}", murmur3_x64_128(test, 0));
    }
}
