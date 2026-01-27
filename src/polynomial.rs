use rand::Rng;
use rand_distr::{Distribution, Normal};

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial {
    pub coefficients: Vec<u64>,
    pub degree: usize,
}

impl Polynomial {
    pub fn new(degree: usize) -> Self {
        Self {
            coefficients: vec![0u64; degree],
            degree,
        }
    }

    pub fn from_coeffs(coeffs: Vec<u64>, modulus: u64) -> Self {
        let degree = coeffs.len();
        let mut reduced_coeffs = coeffs;
        for coeff in &mut reduced_coeffs {
            *coeff %= modulus;
        }

        Self {
            coefficients: reduced_coeffs,
            degree,
        }
    }

    pub fn constant(value: u64, degree: usize) -> Self {
        let mut coeffs = vec![0u64; degree];
        coeffs[0] = value;
        Self {
            coefficients: coeffs,
            degree,
        }
    }

    pub fn random_uniform(degree: usize, modulus: u64) -> Self {
        let mut rng = rand::rng();
        let mut coeffs = vec![0u64; degree];
        for coeff in &mut coeffs {
            *coeff = rng.random_range(0..modulus);
        }

        Self {
            coefficients: coeffs,
            degree,
        }
    }

    pub fn random_binary(degree: usize) -> Self {
        let mut rng = rand::rng();
        let mut coeffs = vec![0u64; degree];
        for coeff in &mut coeffs {
            *coeff = rng.random_range(0..2); // Binary: 0 or 1
        }

        Self {
            coefficients: coeffs,
            degree,
        }
    }

    pub fn random_gaussian(degree: usize, std_dev: f64, modulus: u64) -> Self {
        let normal = Normal::new(0.0, std_dev).unwrap();
        let mut rng = rand::rng();
        let mut coeffs = vec![0u64; degree];

        for coeff in &mut coeffs {
            let sample = normal.sample(&mut rng);
            *coeff = if sample >= 0.0 {
                (sample as u64) % modulus
            } else {
                let neg_sample_abs = (-sample) as u64 % modulus;
                (modulus - neg_sample_abs) % modulus
            };
        }

        Self {
            coefficients: coeffs,
            degree,
        }
    }

    pub fn add(&self, other: &Polynomial, modulus: u64) -> Polynomial {
        debug_assert_eq!(
            self.degree, other.degree,
            "Polynomials must have same degree"
        );

        let mut result = vec![0u64; self.degree];
        for i in 0..self.degree {
            result[i] = (self.coefficients[i] + other.coefficients[i]) % modulus;
        }

        Polynomial {
            coefficients: result,
            degree: self.degree,
        }
    }

    pub fn subtract(&self, other: &Polynomial, modulus: u64) -> Polynomial {
        debug_assert_eq!(
            self.degree, other.degree,
            "Polynomials must have same degree"
        );

        let mut result = vec![0u64; self.degree];
        for i in 0..self.degree {
            result[i] = (self.coefficients[i] + modulus - other.coefficients[i]) % modulus;
        }

        Polynomial {
            coefficients: result,
            degree: self.degree,
        }
    }

    pub fn multiply(&self, other: &Polynomial, modulus: u64) -> Polynomial {
        debug_assert_eq!(self.degree, other.degree, "Polynomials must have same degree");
        
        let max_degree = self.degree * 2 - 1; // Maximum degree for convolution
        let mut result = vec![0u64; self.degree];
        
        for i in 0..self.degree {
            for j in 0..other.degree {
                if self.coefficients[i] != 0 && other.coefficients[j] != 0 {
                    let k = (i + j) % self.degree;
                    let product = self.coefficients[i] as u128 * other.coefficients[j] as u128;
                    let current = result[k] as u128 + product;
                    result[k] = (current % modulus as u128) as u64;
                }
            }
        }
        
        Polynomial {
            coefficients: result,
            degree: self.degree,
        }
    }

    pub fn negacyclic_multiply(&self, other: &Polynomial, modulus: u64) -> Polynomial {
        debug_assert_eq!(
            self.degree, other.degree,
            "Polynomials must have same degree"
        );

        let mut result = vec![0u64; self.degree];

        for i in 0..self.degree {
            for j in 0..self.degree {
                if self.coefficients[i] != 0 && other.coefficients[j] != 0 {
                    let k = (i + j) % self.degree;
                    let product = self.coefficients[i] as u128 * other.coefficients[j] as u128;
                    let current = result[k] as u128 + product;
                    result[k] = (current % modulus as u128) as u64;

                    // Handle negacyclic reduction for x^n + 1
                    if (i + j) >= self.degree {
                        result[k] = (modulus - result[k]) % modulus;
                    }
                }
            }
        }

        Polynomial {
            coefficients: result,
            degree: self.degree,
        }
    }

    pub fn negate(&self, modulus: u64) -> Polynomial {
        let mut result = vec![0u64; self.degree];
        for i in 0..self.degree {
            result[i] = (modulus - self.coefficients[i]) % modulus;
        }

        Polynomial {
            coefficients: result,
            degree: self.degree,
        }
    }

    pub fn mod_reduce(&mut self, modulus: u64) {
        for coeff in &mut self.coefficients {
            *coeff %= modulus;
        }
    }

    pub fn norm(&self) -> u64 {
        *self.coefficients.iter().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod polynomial_tests {
    use super::*;

    #[test]
    fn test_polynomial_construction() {
        let poly = Polynomial::new(8);
        assert_eq!(poly.degree, 8);
        assert_eq!(poly.coefficients, vec![0u64; 8]);

        let const_poly = Polynomial::constant(42, 8);
        assert_eq!(const_poly.coefficients[0], 42);
        assert_eq!(const_poly.coefficients[1..], vec![0u64; 7]);

        let from_vec = Polynomial::from_coeffs(vec![1, 2, 3, 4], 100);
        assert_eq!(from_vec.coefficients, vec![1, 2, 3, 4]);
        assert_eq!(from_vec.degree, 4);
    }

    #[test]
    fn test_polynomial_addition() {
        let p1 = Polynomial::from_coeffs(vec![1, 2, 3], 100);
        let p2 = Polynomial::from_coeffs(vec![4, 5, 6], 100);
        let sum = p1.add(&p2, 100);

        assert_eq!(sum.coefficients, vec![5, 7, 9]);
        assert_eq!(sum.degree, 3);

        // Test modular overflow
        let p3 = Polynomial::from_coeffs(vec![90, 80], 100);
        let p4 = Polynomial::from_coeffs(vec![20, 30], 100);
        let sum2 = p3.add(&p4, 100);
        assert_eq!(sum2.coefficients, vec![10, 10]);
    }

    #[test]
    fn test_polynomial_subtraction() {
        let p1 = Polynomial::from_coeffs(vec![10, 20, 30], 100);
        let p2 = Polynomial::from_coeffs(vec![1, 2, 3], 100);
        let diff = p1.subtract(&p2, 100);

        assert_eq!(diff.coefficients, vec![9, 18, 27]);
        assert_eq!(diff.degree, 3);

        // Test underflow
        let p3 = Polynomial::from_coeffs(vec![5, 10], 100);
        let p4 = Polynomial::from_coeffs(vec![10, 20], 100);
        let diff2 = p3.subtract(&p4, 100);
        assert_eq!(diff2.coefficients, vec![95, 90]);
    }

    #[test]
    fn test_polynomial_multiplication() {
        let p1 = Polynomial::from_coeffs(vec![1, 2], 100);
        let p2 = Polynomial::from_coeffs(vec![3, 4], 100);
        let product = p1.multiply(&p2, 100);

        // (1 + 2x) * (3 + 4x) = 3 + 10x + 8x^2
        println!("Product: {:?}", product.coefficients);
        let expected_coeffs = vec![3, 10];
        assert_eq!(product.coefficients[..2], expected_coeffs);
        assert_eq!(product.degree, 2);
    }

    #[test]
    fn test_negacyclic_properties() {
        // Test negacyclic multiplication for x^4 + 1
        let p1 = Polynomial::from_coeffs(vec![0, 0, 0, 1], 100); // x^3
        let p2 = Polynomial::from_coeffs(vec![0, 0, 0, 1], 100); // x^3

        let result = p1.negacyclic_multiply(&p2, 100);

        // x^3 * x^3 = x^6 = x^2 * (x^4 + 1) - x^2 = -x^2 mod (x^4 + 1)
        let expected_coeff = 100 - 1;
        assert_eq!(result.coefficients, vec![0, 0, expected_coeff, 0]);
    }

    #[test]
    fn test_random_generation() {
        let uniform_poly = Polynomial::random_uniform(100, 1000);
        assert_eq!(uniform_poly.degree, 100);
        assert!(uniform_poly.coefficients.iter().all(|&x| x < 1000));

        let binary_poly = Polynomial::random_binary(50);
        assert_eq!(binary_poly.degree, 50);
        assert!(binary_poly.coefficients.iter().all(|&x| x == 0 || x == 1));

        let gaussian_poly = Polynomial::random_gaussian(100, 3.2, 1000);
        assert_eq!(gaussian_poly.degree, 100);
        assert!(gaussian_poly.coefficients.iter().all(|&x| x < 1000));
    }

    #[test]
    fn test_negate() {
        let poly = Polynomial::from_coeffs(vec![10, 20, 30], 100);
        let negated = poly.negate(100);

        assert_eq!(negated.coefficients, vec![90, 80, 70]);
    }

    #[test]
    fn test_norm() {
        let poly = Polynomial::from_coeffs(vec![10, 50, 30, 70], 100);
        assert_eq!(poly.norm(), 70);

        let zero_poly = Polynomial::new(4);
        assert_eq!(zero_poly.norm(), 0);
    }

    #[test]
    fn test_mod_reduce() {
        let mut poly = Polynomial::from_coeffs(vec![150, 250, 350], 100);
        poly.mod_reduce(100);

        assert_eq!(poly.coefficients, vec![50, 50, 50]);
    }
}
