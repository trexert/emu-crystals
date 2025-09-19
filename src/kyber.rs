use num::Integer;
use rand;
use sha3::{
    Sha3_256, Sha3_512, Shake128,
    digest::{ExtendableOutput, Update, XofReader},
};
use std::marker::ConstParamTy;

pub type PrivateKey<const P: ParameterSet> = [u8; P.private_key_size()];
pub type PublicKey<const P: ParameterSet> = [u8; P.public_key_size()];
pub type EncodedPolynomial<const P: ParameterSet> = [u8; P.encoded_polynomial_bytes()];

pub struct KeyPair<const P: ParameterSet>
where
    [(); P.private_key_size()]:,
    [(); P.public_key_size()]:,
{
    private: Box<PrivateKey<P>>,
    public: Box<PublicKey<P>>,
}

impl<const P: ParameterSet> KeyPair<P>
where
    [(); P.private_key_size()]:,
    [(); P.public_key_size()]:,
{
    pub fn generate(rng: &mut impl rand::RngCore) -> Self {
        let mut rho = [0u8; 32];
        let mut sigma = [0u8; 32];
        rng.fill_bytes(&mut rho);
        rng.fill_bytes(&mut sigma);

        panic!("Not implemented")
    }
}

#[derive(Clone)]
struct Ring<const P: ParameterSet>
where
    [(); P.polynomial_order()]:,
{
    coefficients: [u32; P.polynomial_order()],
}

impl<const P: ParameterSet> Ring<P>
where
    [(); P.polynomial_order()]:,
    [(); P.encoded_polynomial_bytes()]:,
{
    // Decode from byte array of form [aaaaaaaa,aaaabbbb,bbbbbbbb,cccccccc,ccccdddd,dddddddd] etc
    pub fn decode(bytes: &EncodedPolynomial<P>) -> Self {
        let mut coefficients = [0u32; P.polynomial_order()];
        let mut current_bit = 0;
        let mut current_byte = 0;
        for i in 0..P.polynomial_order() {
            let mut bits_needed = P.q_bits();
            let (next_byte, next_bit) = ((i + 1) * P.q_bits()).div_mod_floor(&8);
            while current_byte < next_byte {
                let bits = 8 - current_bit;
                let mut bitmask = 0u8;
                while current_bit < 8 {
                    bitmask |= 1 << (7 - current_bit);
                    current_bit += 1
                }
                coefficients[i] |= ((bytes[current_byte] & bitmask) as u32) << (bits_needed - bits);
                bits_needed -= bits;
                current_bit = 0;

                current_byte += 1;
            }

            assert!(bits_needed == next_bit - current_bit);
            if current_bit < next_bit {
                let mut bitmask = 0u8;
                while current_bit < next_bit {
                    bitmask |= 1 << (7 - current_bit);
                    current_bit += 1
                }
                coefficients[i] |= ((bytes[current_byte] & bitmask) as u32) >> current_bit;
            }
        }
        Self { coefficients }
    }

    pub fn encode() -> EncodedPolynomial<P> {
        let mut encoded = [0u8; P.encoded_polynomial_bytes()];
    }
}

// fn generate_matrix<const K: usize>(rho: &[u8; 32]) -> Vec<Vec<Ring>> {
//     let mut matrix: Vec<Vec<Ring>> = vec![Vec::with_capacity(K); K];
//     let mut bytes = [0u8; Q_BITS * N / 8];
//     for i in 0..K {
//         for j in 0..K {
//             let mut shake = Shake128::default();
//             shake.update(rho);
//             shake.update(i);
//             shake.update(j);
//             let mut xof = shake.finalize_xof();
//             xof.read(&mut bytes);
//             matrix[i][j] = Ring::decode(bytes);
//         }
//     }

//     panic!("Not implemented")
// }

trait BinomialNoise {
    fn binomial_noise(&mut self, eta: u8) -> i8;
}

impl<T: rand::RngCore> BinomialNoise for T {
    fn binomial_noise(&mut self, eta: u8) -> i8 {
        assert!(eta < 128); // Values must fit in an i8
        let mut bytes = vec![0; (eta * 2).div_ceil(8) as usize];
        self.fill_bytes(&mut bytes);
        let mut a = 0;
        let mut b = 0;
        for i in 0..eta {
            let (byte, bit) = (2 * i).div_mod_floor(&8);
            a += (bytes[byte as usize] & 1 << bit) >> bit;
            b += (bytes[byte as usize] & 1 << (bit + 1)) >> (bit + 1);
        }
        a as i8 - b as i8
    }
}

#[derive(ConstParamTy, PartialEq, Eq, Clone, Copy)]
struct ParameterSet {
    n: u16,
    k: u8,
    q: u32,
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

    const fn encoded_polynomial_bytes(self) -> usize {
        (self.n as usize * self.q_bits()).div_ceil(8)
    }

    const fn q_bits(self) -> usize {
        (size_of_val(&self.q) as u32 * 8 - self.q.leading_zeros()) as usize
    }
}

pub const K512: ParameterSet = ParameterSet {
    n: 256,
    k: 2,
    q: 3329,
    eta1: 3,
    eta2: 2,
    du: 10,
    dv: 4,
};

pub const K768: ParameterSet = ParameterSet {
    n: 256,
    k: 3,
    q: 3329,
    eta1: 2,
    eta2: 2,
    du: 10,
    dv: 4,
};

pub const K1024: ParameterSet = ParameterSet {
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

    const TEST_PARAMS: ParameterSet = ParameterSet {
        n: 2,
        k: 2,
        q: 3329,
        eta1: 2,
        eta2: 2,
        du: 10,
        dv: 2,
    };

    const SEED: [u8; 32] = [
        143, 251, 241, 213, 226, 46, 234, 11, 69, 162, 59, 195, 240, 113, 105, 158, 70, 73, 137,
        154, 104, 1, 123, 91, 230, 48, 198, 178, 13, 122, 174, 105,
    ];

    #[test]
    fn test_binomial_noise() {
        const ETA: u8 = 9;
        let mut rng = ChaCha8Rng::from_seed(SEED);
        let mut results = [0u32; (ETA * 2 + 1) as usize];
        for _ in 0..100_000 {
            results[(rng.binomial_noise(ETA) + ETA as i8) as usize] += 1;
        }
        println!("{:?}", results);
        for i in 0..ETA as usize {
            assert!(results[i] < results[i + 1]);
            assert!(results[2 * ETA as usize - i] < results[2 * ETA as usize - i - 1]);
        }
    }

    #[test]
    fn test_decode() {
        let ring = Ring::<TEST_PARAMS>::decode(&[0b00000010, 0b01010100, 0b00000101]);
        assert!(ring.coefficients == [37, 1029]);
    }
}
