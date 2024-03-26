use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pathfinder_crypto::hash::pedersen::pedersen_hash;
use pathfinder_crypto::Felt as PFelt;
use rand::{rngs::StdRng, RngCore, SeedableRng};
use starknet_types_core::felt::Felt;
use starknet_types_core::hash::Pedersen;
use starknet_types_core::hash::StarkHash;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = StdRng::from_entropy();

    let mut felt1: [u8; 31] = Default::default();
    rng.fill_bytes(&mut felt1);

    let mut felt2: [u8; 31] = Default::default();
    rng.fill_bytes(&mut felt2);

    let lw_felt1 = Felt::from_bytes_be_slice(&felt1);
    let lw_felt2 = Felt::from_bytes_be_slice(&felt2);

    let pf_felt1 = PFelt::from_be_slice(&felt1).unwrap();
    let pf_felt2 = PFelt::from_be_slice(&felt2).unwrap();
    c.bench_function("LW", |b| {
        b.iter(|| {
            black_box(Pedersen::hash(&lw_felt1, &lw_felt2));
            // pedersen_hash(felt1, felt2);
        })
    });

    c.bench_function("Pathfinder", |b| {
        b.iter(|| {
            black_box(pedersen_hash(pf_felt1, pf_felt2));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
