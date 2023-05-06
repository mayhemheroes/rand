#![no_main]
use libfuzzer_sys::fuzz_target;
use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng};
use rand_distr::{Normal, Distribution, Weibull, LogNormal, Pareto, ChiSquared};


fuzz_target!(|input: (f64, f64)| {
    let (a, b) = input;
    let mut rng = ChaCha20Rng::seed_from_u64(0);
    match Normal::new(a, b) {
        Ok(dst) => {
            (0..10000).into_iter().for_each(|_| { dst.sample(&mut rng); });
        },
        Err(_) => {},
    }
    
    match Weibull::new(a, b) {
        Ok(dst) => {
            (0..10000).into_iter().for_each(|_| { dst.sample(&mut rng); });
        },
        Err(_) => {},
    }
    
    match LogNormal::new(a, b) {
        Ok(dst) => {
            (0..10000).into_iter().for_each(|_| { dst.sample(&mut rng); });
        },
        Err(_) => {},
    }

    match Pareto::new(a, b) {
        Ok(dst) => {
            (0..10000).into_iter().for_each(|_| { dst.sample(&mut rng); });
        },
        Err(_) => {},
    }

    match ChiSquared::new(a) {
        Ok(dst) => {
            (0..10000).into_iter().for_each(|_| { dst.sample(&mut rng); });
        },
        Err(_) => {},
    }
});