use project_euler::util::prime;
use test::Bencher;

use rand::{Rng, SeedableRng};

#[bench]
fn trial_division(bench: &mut Bencher) {
    const MIN: u64 = 1;
    const MAX: u64 = 1_000_000;
    let mut prng = rand_pcg::Pcg32::seed_from_u64(42);
    bench.iter(|| prime::checks::trial_division(prng.gen_range(MIN, MAX)))
}

#[bench]
fn sieve(bench: &mut Bencher) {
    const MIN: u64 = 1;
    const MAX: u64 = 1_000_000;
    let sieve = prime::sieve(MAX as usize);

    let mut prng = rand_pcg::Pcg32::seed_from_u64(42);
    bench.iter(|| sieve[prng.gen_range(MIN, MAX) as usize])
}
