use rand;
use sha3::{
    Sha3_256, Sha3_512, Shake128,
    digest::{ExtendableOutput, Update, XofReader},
};
use std::marker::ConstParamTy;

pub type PrivateKey<const P: ParameterSet> = [u8; P.private_key_size()];
pub type PublicKey<const P: ParameterSet> = [u8; P.public_key_size()];

pub struct KeyPair<const P: ParameterSet>
where
    [(); P.private_key_size()]:,
    [(); P.public_key_size()]:,
{
    private: PrivateKey<P>,
    public: PublicKey<P>,
}

impl <const P: ParameterSet> KeyPair<P>
where
    [(); P.private_key_size()]:,
    [(); P.public_key_size()]:,
{
    
}

#[derive(Clone)]
struct Ring<const P: ParameterSet>
where
    [(); P.polynomial_order()]:,
{
    coefficients: [u16; P.polynomial_order()],
}

impl Ring<const P: ParameterSet> {
    fn decode(bytes: [u8; Q_BITS * N / 8]) -> Self {
        panic!("Not implemented")
    }
}

pub fn generate_key<const K: usize>(rng: &mut impl rand::RngCore) -> KeyPair<K>
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

fn generate_matrix<const K: usize>(rho: &[u8; 32]) -> Vec<Vec<Ring>> {
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

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
pub struct ParameterSet {
    n: u16,
    k: u8,
    q: u16,
    eta1: u8,
    eta2: u8,
    du: u8,
    dv: u8,
}

impl ParameterSet {
    const fn private_key_size(self) -> usize {
        (self.k as usize * self.n as usize * self.q_bits()).div_ceil(8)
    }

    const fn public_key_size(self) -> usize {
        self.private_key_size() + 32
    }

    const fn polynomial_order(self) -> usize {
        self.n as usize
    }

    const fn q_bits(self) -> usize {
        size_of_val(&self.q) - self.q.leading_zeros() as usize
    }
}

const K512: ParameterSet = ParameterSet {
    n: 256,
    k: 2,
    q: 3329,
    eta1: 3,
    eta2: 2,
    du: 10,
    dv: 4,
};

const K768: ParameterSet = ParameterSet {
    n: 256,
    k: 3,
    q: 3329,
    eta1: 2,
    eta2: 2,
    du: 10,
    dv: 4,
};

const K1024: ParameterSet = ParameterSet {
    n: 256,
    k: 3,
    q: 3329,
    eta1: 2,
    eta2: 2,
    du: 11,
    dv: 5,
};

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
