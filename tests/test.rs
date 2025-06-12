#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const MAX_TESTS: u64 = 1000;
    const MAX_TEXT_LENGTH: usize = 256;
    const SEED: u32 = 0;

    fn lcg(seed: &mut u64) -> u64 {
        *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        *seed
    }

    fn generate_random_string(mut seed: u64, length: usize) -> String {
        let mut result = String::with_capacity(length);

        for _ in 0..length {
            let rand = lcg(&mut seed);
            let index = (rand % CHARSET.len() as u64) as usize;
            result.push(CHARSET[index] as char);
        }

        result
    }

    fn run_hash_test<F1, F2, R>(mut hash1_fn: F1, mut hash2_fn: F2)
    where
        F1: FnMut(&[u8]) -> R,
        F2: FnMut(&[u8]) -> R,
        R: PartialEq + Debug,
    {
        for mut i in 0..MAX_TESTS {
            let length = (lcg(&mut i) as usize) % (MAX_TEXT_LENGTH + 1) + 1;
            let text = generate_random_string(i, length);
            let text_bytes = text.as_bytes();

            let hash1 = hash1_fn(text_bytes);
            let hash2 = hash2_fn(text_bytes);

            assert_eq!(
                hash1, hash2,
                "Hash mismatch for input: \"{}\" (len {})\n{:?} != {:?}",
                text, length, hash1, hash2
            );
        }
    }

    #[test]
    fn hash_equal_x64_128() {
        run_hash_test(
            |text| murmur3_rust::murmur3_x64_128(text, SEED),
            |text| murmur3_binding::murmur3_x64_128(text, SEED),
        );
    }

    #[test]
    fn hash_equal_x86_128() {
        run_hash_test(
            |text| murmur3_rust::murmur3_x86_128(text, SEED),
            |text| murmur3_binding::murmur3_x86_128(text, SEED),
        );
    }

    #[test]
    fn hash_equal_x86_32() {
        run_hash_test(
            |text| murmur3_rust::murmur3_x86_32(text, SEED),
            |text| murmur3_binding::murmur3_x86_32(text, SEED),
        );
    }
}
