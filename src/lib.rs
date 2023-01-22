#![deny(unsafe_code)]

use rand_core::{RngCore};

pub const HASH_BYTE_SIZE: usize = 32;
pub type HashOutput = [u8; HASH_BYTE_SIZE];

pub fn get_random_bytes<const N: usize>(rng: &mut impl RngCore) -> [u8; N] {
    let mut bytes = [0u8; N];
    rng.fill_bytes(&mut bytes);
    bytes
}

pub fn generate_double_random_safe(rng: &mut impl RngCore) -> (HashOutput, HashOutput) {
    (
        get_random_bytes::<HASH_BYTE_SIZE>(rng),
        get_random_bytes::<HASH_BYTE_SIZE>(rng),
    )
}

#[allow(unsafe_code)]
pub fn generate_double_random_unsafe(rng: &mut impl RngCore) -> (HashOutput, HashOutput) {
    const _DOUBLE_SIZE: usize = HASH_BYTE_SIZE * 2;
    let mut all = get_random_bytes::<_DOUBLE_SIZE>(rng);

    // infallible
    assert_eq!(all.len(), 64);

    unsafe {
        let ptr = all.as_mut_ptr();
        (
            std::slice::from_raw_parts_mut(ptr, 32)
                .try_into()
                .expect("generate_double_random Failure on first"),
            std::slice::from_raw_parts_mut(ptr.add(32), 64 - 32)
                .try_into()
                .expect("generate_double_random Failure on second"),
        )
    }
}
