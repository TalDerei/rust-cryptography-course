// Import crates
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rsa::{
    errors::Error,
    rand_core::{CryptoRng, RngCore},
    PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey,
};
use sha2::{Digest, Sha256};

pub fn encrypt<R: RngCore + CryptoRng>(
    rng: &mut R,
    pub_key: &RsaPublicKey,
    data: &[u8],
) -> Result<Vec<u8>, Error> {
    Ok(pub_key
        .encrypt(rng, PaddingScheme::new_oaep::<Sha256>(), data)
        .expect("failed to encrypt")
    )
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    let data = b"test";

    c.bench_function("encrypt-bench", |b| {
        b.iter(|| encrypt(&mut rng, &pub_key, data))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
