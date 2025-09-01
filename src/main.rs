#![feature(generic_const_exprs)]
use rand;
use sha3::{
    Sha3_256, Sha3_512, Shake128,
    digest::{ExtendableOutput, Update, XofReader},
};

const N: usize = 256;
const Q: usize = 3329;
const Q_BITS: usize = size_of::<usize>() * 8 - Q.leading_zeros() as usize;

fn main() {
    println!("{}", size_of::<Key<1024>>())
}

pub struct Key<const K: u32>
where
    [(); Q_BITS * K * N / 8]:,
    [(); Q_BITS * K * N / 8 + 32]:,
{
    private: [u8; Q_BITS * K * N / 8],
    public: [u8; Q_BITS * K * N / 8 + 32],
}

#[derive(Clone)]
struct Ring {
    coefficients: [usize; N],
}

impl Ring {
    fn decode(bytes: [u8; Q_BITS * N / 8]) -> Self {}
}

pub fn generate_key<const K: u32>(rng: &mut impl rand::RngCore) -> Key<K>
where
    [(); Q_BITS * K * N / 8]:,
    [(); Q_BITS * K * N / 8 + 32]:,
{
    let mut rho = [0u8; 32];
    let mut sigma = [0u8; 32];
    rng.fill_bytes(&mut rho);
    rng.fill_bytes(&mut sigma);

    panic!("Not implemented")
}

fn generate_matrix<const K: u32>(rho: &[u8; 32]) -> Vec<Vec<Ring>> {
    let mut matrix: Vec<Vec<Ring>> = vec![Vec::with_capacity(K); K];
    let mut bytes = [0u8; Q_BITS * N / 8];
    for i in 0..K {
        for j in 0..K {
            let mut shake = Shake128::default();
            shake.update(rho);
            shake.update(i);
            shake.update(j);
            let mut xof = shake.finalize_xof();
            xof.read(&mut bytes);
            matrix[i][j] = Ring::decode(bytes);
        }
    }

    panic!("Not implemented")
}

trait BinomialNoise {
    fn binomial_noise(&mut self, eta: u8) -> u8;
}

impl<T: rand::RngCore> BinomialNoise for T {
    fn binomial_noise(&mut self, eta: u8) -> u8 {
        let eta = eta + 1;
        let a = (self.next_u32() % eta as u32) as u8;
        let b = (self.next_u32() % eta as u32) as u8;
        a.wrapping_sub(b)
    }
}

#[cfg(test)]
mod test {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    const SEED: [u8; 32] = [
        143, 251, 241, 213, 226, 46, 234, 11, 69, 162, 59, 195, 240, 113, 105, 158, 70, 73, 137,
        154, 104, 1, 123, 91, 230, 48, 198, 178, 13, 122, 174, 105,
    ];

    #[test]
    fn test_binomial_noise() {
        let mut rng = ChaCha8Rng::from_seed(SEED);
        let mut results = [0u8; 9];
        for _ in 0..1024 {
            results[(rng.binomial_noise(4).wrapping_add(4)) as usize] += 1;
        }
        println!("{:?}", results);
        assert!(results[0] > 0);
        assert!(results[0] < results[1]);
        assert!(results[1] < results[2]);
        assert!(results[2] < results[3]);
        assert!(results[3] < results[4]);
        assert!(results[4] > results[5]);
        assert!(results[5] > results[6]);
        assert!(results[6] > results[7]);
        assert!(results[7] > results[8]);
        assert!(results[8] > 0);
    }
}
