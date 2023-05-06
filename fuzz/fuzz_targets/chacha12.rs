#![no_main]
use libfuzzer_sys::fuzz_target;
use rand_chacha::{ChaCha12Rng, rand_core::{SeedableRng, RngCore}};

fuzz_target!(|input: (u8, u64, Vec<u8>, u8, u64, u128)| {
    let (n_to_generate, seed, vec_seed, mut e, stream, word_offset) = input;
    let mut rng = ChaCha12Rng::seed_from_u64(seed);
    rng.set_stream(stream);
    rng.set_word_pos(word_offset);
    for i in 0..n_to_generate {
        e = e.wrapping_add(e.wrapping_pow(i as u32));
        if e % 2 == 0 {
            rng.next_u32();
        } else {
            rng.next_u64();
        }
    }

    let min = vec_seed.len().min(32);
    let mut seed_slice: [u8; 32] = [0; 32];
    seed_slice[0..min].copy_from_slice(&vec_seed[0..min]);

    let mut rng = ChaCha12Rng::from_seed(seed_slice);
    rng.set_stream(stream);
    rng.set_word_pos(word_offset);
    for i in 0..n_to_generate {
        e = e.wrapping_add(e.wrapping_pow(i as u32));
        if e % 2 == 0 {
            rng.next_u32();
        } else {
            rng.next_u64();
        }
    }
});