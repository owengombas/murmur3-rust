use std::usize;

fn fmix32(h: u32) -> u32 {
    let mut h = h;
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;
    h
}

fn fmix64(h: u64) -> u64 {
    let mut h = h;
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51afd7ed558ccd_u64);
    h ^= h >> 33;
    h = h.wrapping_mul(0xc4ceb9fe1a85ec53_u64);
    h ^= h >> 33;
    h
}

pub fn murmur3_x86_32(key: &[u8], seed: u32) -> u32 {
    const DATA_SIZE: usize = 4;
    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;

    let len = key.len();
    let n_blocks: usize = len / DATA_SIZE;

    let mut h1: u32 = seed;

    for i in 0..n_blocks {
        let base = i.wrapping_mul(DATA_SIZE);
        let mut k1 = u32::from_le_bytes(key[base..base + DATA_SIZE].try_into().unwrap());

        k1 = k1.wrapping_mul(C1).rotate_left(15).wrapping_mul(C2);

        h1 ^= k1;
        h1 = h1.rotate_left(13).wrapping_mul(5).wrapping_add(0xe6546b64);
    }

    let tail = &key[n_blocks * DATA_SIZE..];
    let mut k1: u32 = 0;
    let len3 = len & 3;

    if len3 >= 3 {
        k1 ^= (tail[2] as u32) << 16;
    }
    if len3 >= 2 {
        k1 ^= (tail[1] as u32) << 8;
    }
    if len3 >= 1 {
        k1 ^= tail[0] as u32;
        k1 = k1.wrapping_mul(C1).rotate_left(15).wrapping_mul(C2);
        h1 ^= k1;
    }

    h1 ^= u32::try_from(len).unwrap();

    h1 = fmix32(h1);

    h1
}

pub fn murmur3_x86_128(key: &[u8], seed: u32) -> [u32; 4] {
    const DATA_SIZE: usize = 16;
    const C1: u32 = 0x239b961b;
    const C2: u32 = 0xab0e9789;
    const C3: u32 = 0x38b34ae5;
    const C4: u32 = 0xa1e38b93;

    let len = key.len();
    let n_blocks: usize = len / DATA_SIZE;

    let mut h1: u32 = seed;
    let mut h2: u32 = h1;
    let mut h3: u32 = h1;
    let mut h4: u32 = h1;

    for i in 0..n_blocks {
        let base = i.wrapping_mul(DATA_SIZE);
        let mut k1 = u32::from_le_bytes(key[base..base + 4].try_into().unwrap());
        let mut k2 = u32::from_le_bytes(key[base + 4..base + 8].try_into().unwrap());
        let mut k3 = u32::from_le_bytes(key[base + 8..base + 12].try_into().unwrap());
        let mut k4 = u32::from_le_bytes(key[base + 12..base + 16].try_into().unwrap());

        k1 = k1.wrapping_mul(C1).rotate_left(15).wrapping_mul(C2);
        h1 ^= k1;
        h1 = h1
            .rotate_left(19)
            .wrapping_add(h2)
            .wrapping_mul(5)
            .wrapping_add(0x561ccd1b);

        k2 = k2.wrapping_mul(C2).rotate_left(16).wrapping_mul(C3);
        h2 ^= k2;
        h2 = h2
            .rotate_left(17)
            .wrapping_add(h3)
            .wrapping_mul(5)
            .wrapping_add(0x0bcaa747);

        k3 = k3.wrapping_mul(C3).rotate_left(17).wrapping_mul(C4);
        h3 ^= k3;
        h3 = h3
            .rotate_left(15)
            .wrapping_add(h4)
            .wrapping_mul(5)
            .wrapping_add(0x96cd1c35);

        k4 = k4.wrapping_mul(C4).rotate_left(18).wrapping_mul(C1);
        h4 ^= k4;
        h4 = h4
            .rotate_left(13)
            .wrapping_add(h1)
            .wrapping_mul(5)
            .wrapping_add(0x32ac3b17);
    }

    let tail = &key[n_blocks * DATA_SIZE..];
    let len15 = len & 15;

    let mut k1: u32 = 0;
    let mut k2: u32 = 0;
    let mut k3: u32 = 0;
    let mut k4: u32 = 0;

    if len15 >= 15 {
        k4 ^= (tail[14] as u32) << 16;
    }
    if len15 >= 14 {
        k4 ^= (tail[13] as u32) << 8;
    }
    if len15 >= 13 {
        k4 ^= (tail[12] as u32) << 0;
        k4 = k4.wrapping_mul(C4).rotate_left(18).wrapping_mul(C1);
        h4 ^= k4;
    }

    if len15 >= 12 {
        k3 ^= (tail[11] as u32) << 24;
    }
    if len15 >= 11 {
        k3 ^= (tail[10] as u32) << 16;
    }
    if len15 >= 10 {
        k3 ^= (tail[9] as u32) << 8;
    }
    if len15 >= 9 {
        k3 ^= (tail[8] as u32) << 0;
        k3 = k3.wrapping_mul(C3).rotate_left(17).wrapping_mul(C4);
        h3 ^= k3;
    }

    if len15 >= 8 {
        k2 ^= (tail[7] as u32) << 24;
    }
    if len15 >= 7 {
        k2 ^= (tail[6] as u32) << 16;
    }
    if len15 >= 6 {
        k2 ^= (tail[5] as u32) << 8;
    }
    if len15 >= 5 {
        k2 ^= (tail[4] as u32) << 0;
        k2 = k2.wrapping_mul(C2).rotate_left(16).wrapping_mul(C3);
        h2 ^= k2;
    }

    if len15 >= 4 {
        k1 ^= (tail[3] as u32) << 24;
    }
    if len15 >= 3 {
        k1 ^= (tail[2] as u32) << 16;
    }
    if len15 >= 2 {
        k1 ^= (tail[1] as u32) << 8;
    }
    if len15 >= 1 {
        k1 ^= (tail[0] as u32) << 0;
        k1 = k1.wrapping_mul(C1).rotate_left(15).wrapping_mul(C2);
        h1 ^= k1;
    }

    let len32 = u32::try_from(len).unwrap();
    h1 ^= len32;
    h2 ^= len32;
    h3 ^= len32;
    h4 ^= len32;

    h1 = h1.wrapping_add(h2);
    h1 = h1.wrapping_add(h3);
    h1 = h1.wrapping_add(h4);
    h2 = h2.wrapping_add(h1);
    h3 = h3.wrapping_add(h1);
    h4 = h4.wrapping_add(h1);

    h1 = fmix32(h1);
    h2 = fmix32(h2);
    h3 = fmix32(h3);
    h4 = fmix32(h4);

    h1 = h1.wrapping_add(h2);
    h1 = h1.wrapping_add(h3);
    h1 = h1.wrapping_add(h4);
    h2 = h2.wrapping_add(h1);
    h3 = h3.wrapping_add(h1);
    h4 = h4.wrapping_add(h1);

    [h1, h2, h3, h4]
}

pub fn murmur3_x64_128(key: &[u8], seed: u32) -> [u64; 2] {
    const DATA_SIZE: usize = 16;
    const C1: u64 = 0x87c37b91114253d5;
    const C2: u64 = 0x4cf5ad432745937f;

    let len = key.len();
    let n_blocks: usize = len / DATA_SIZE;

    let mut h1: u64 = seed as u64;
    let mut h2: u64 = h1;

    for i in 0..n_blocks {
        let base = i.wrapping_mul(DATA_SIZE);
        let mut k1 = u64::from_le_bytes(key[base..base + 8].try_into().unwrap());
        let mut k2 = u64::from_le_bytes(key[base + 8..base + 16].try_into().unwrap());

        k1 = k1.wrapping_mul(C1).rotate_left(31).wrapping_mul(C2);
        h1 ^= k1;
        h1 = h1
            .rotate_left(27)
            .wrapping_add(h2)
            .wrapping_mul(5)
            .wrapping_add(0x52dce729);

        k2 = k2.wrapping_mul(C2).rotate_left(33).wrapping_mul(C1);
        h2 ^= k2;
        h2 = h2
            .rotate_left(31)
            .wrapping_add(h1)
            .wrapping_mul(5)
            .wrapping_add(0x38495ab5);
    }

    let tail = &key[n_blocks * DATA_SIZE..];
    let len15 = len & 15;

    let mut k1: u64 = 0;
    let mut k2: u64 = 0;

    if len15 >= 15 {
        k2 ^= (tail[14] as u64) << 48;
    }
    if len15 >= 14 {
        k2 ^= (tail[13] as u64) << 40;
    }
    if len15 >= 13 {
        k2 ^= (tail[12] as u64) << 32;
    }
    if len15 >= 12 {
        k2 ^= (tail[11] as u64) << 24;
    }
    if len15 >= 11 {
        k2 ^= (tail[10] as u64) << 16;
    }
    if len15 >= 10 {
        k2 ^= (tail[9] as u64) << 8;
    }
    if len15 >= 9 {
        k2 ^= (tail[8] as u64) << 0;
        k2 = k2.wrapping_mul(C2).rotate_left(33).wrapping_mul(C1);
        h2 ^= k2;
    }

    if len15 >= 8 {
        k1 ^= (tail[7] as u64) << 56;
    }
    if len15 >= 7 {
        k1 ^= (tail[6] as u64) << 48;
    }
    if len15 >= 6 {
        k1 ^= (tail[5] as u64) << 40;
    }
    if len15 >= 5 {
        k1 ^= (tail[4] as u64) << 32;
    }
    if len15 >= 4 {
        k1 ^= (tail[3] as u64) << 24;
    }
    if len15 >= 3 {
        k1 ^= (tail[2] as u64) << 16;
    }
    if len15 >= 2 {
        k1 ^= (tail[1] as u64) << 8;
    }
    if len15 >= 1 {
        k1 ^= (tail[0] as u64) << 0;
        k1 = k1.wrapping_mul(C1).rotate_left(31).wrapping_mul(C2);
        h1 ^= k1;
    }

    let len64 = u64::try_from(len).unwrap();
    h1 ^= len64;
    h2 ^= len64;

    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);

    h1 = fmix64(h1);
    h2 = fmix64(h2);

    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);

    [h1, h2]
}
