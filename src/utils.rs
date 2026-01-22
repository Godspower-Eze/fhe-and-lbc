use rand::{Rng, TryRngCore, rngs::OsRng};
use rand_distr::{Normal};

pub fn add_vec(vec_1: &Vec<i128>, vec_2: &Vec<i128>, q: i128) -> Vec<i128> {
    let mut resulting_vec = vec![];
    for (e_1, e_2) in vec_1.iter().zip(vec_2) {
        let sum = (e_1 + e_2).rem_euclid(q);
        resulting_vec.push(sum);
    }
    resulting_vec
}

pub fn inner_product_and_add(vec_1: &Vec<i128>, vec_2: &Vec<i128>, s:i128, q: i128) -> i128 {
    let mut result = 0;
    for (e_1, e_2) in vec_1.iter().zip(vec_2) {
        result += ((e_1 * e_2) + s).rem_euclid(q);
    }
    result.rem_euclid(q)
}

pub fn generate_random_vector(n:usize, q: usize) -> Vec<i128> {
    let mut vector = vec![];
    let mut rng = [0u8; 16];
    for _ in 0..n {
        OsRng.try_fill_bytes(&mut rng).unwrap();
        let random_val = OsRng.try_next_u64().unwrap() as i128;
        vector.push(((random_val as usize) % q).try_into().unwrap());
    }
    vector
}

pub fn generate_random_matrix(m: usize, n: usize, q: usize) -> Vec<Vec<i128>> {
    let mut matrix = vec![];
    for _ in 0..m {
        let row = generate_random_vector(n, q);
        matrix.push(row);
    }
    matrix
}

pub fn generate_random_bit_vector(m: usize) -> Vec<i128> {
    generate_random_vector(m, 2)
}

pub fn sample_discrete_gaussian(sigma: f64) -> i128 {
    let mut rng = rand::rng();
    let normal: f64 = rng.sample(Normal::new(0.0, sigma).unwrap());
    normal.round() as i128
}

pub fn sample_discrete_gaussian_vector(sigma: f64, m: usize) -> Vec<i128> {
    (0..m).map(|_| sample_discrete_gaussian(sigma).try_into().unwrap()).collect()
}

pub fn transpose_matrix(a: &Vec<Vec<i128>>) -> Vec<Vec<i128>> {
    let row = a.len();
    let column = a[0].len();
    let mut transposed_matrix = vec![vec![0; row]; column];
    for i in 0..row {
        for j in 0..column {
            let element = a[i][j];
            transposed_matrix[j][i] = element;
        }
    }
    transposed_matrix
}

pub fn matrix_mul_vector(a: &Vec<Vec<i128>>, b: &Vec<i128>, q: i128) -> Vec<i128> {
    debug_assert!(a[0].len() == b.len(), "column of a should be equal to row of b");

    let row = a.len();
    let column = a[0].len();
    let mut vector = vec![0; row];

    for i in 0..row {
        let mut sum = 0;
        for j in 0..column {
            sum += a[i][j] * b[j];
        }
        vector[i] = sum % q;     // mod q
    }

    vector
}

pub fn center_mod(val: i128, q: i128) -> i128 {
    let mut v = val.rem_euclid(q);
    if v > q/2 {  // map large positives to negative
        v -= q;
    }
    v
}

pub fn mod_inv(a: i128, q: i128) -> Option<i128> {
    let (mut a, mut m0) = (a, q);
    let (mut x0, mut x1) = (0, 1);

    if q == 1 { return Some(0); }
    while a > 1 {
        println!("{}", m0);
        let q = a / m0;
        let mut t = m0;

        m0 = a % m0;
        a = t;
        t = x0;

        x0 = x1 - q * x0;
        x1 = t;
    }

    if x1 < 0 { x1 += q; }
    Some(x1)
}

pub fn sieve_primes(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return vec![];
    }

    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            let mut multiple = i * i;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += i;
            }
        }
    }

    is_prime.iter()
            .enumerate()
            .filter_map(|(i, &prime)| if prime { Some(i) } else { None })
            .collect()
}

pub fn generate_primes(n: usize) -> Vec<i128> {
    let mut primes = vec![];
    let mut candidate = 2;

    while primes.len() < n {
        if primes.iter().all(|p| candidate % p != 0) {
            primes.push(candidate as i128);
        }
        candidate += 1;
    }

    primes
}

#[cfg(test)]
mod test {

    use crate::utils::{center_mod, generate_random_bit_vector, inner_product_and_add, matrix_mul_vector, mod_inv, transpose_matrix};


    #[test]
    fn test_center_mod(){
        let q = 11;
        let elements = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut adjusted_elements = vec![];
        for i in 0..q {
            adjusted_elements.push(center_mod(elements[i], q as i128));
        }
        let expected_adjusted_elements = vec![0, 1, 2, 3, 4, 5, -5, -4, -3, -2, -1];
        assert_eq!(adjusted_elements, expected_adjusted_elements)
    }

    #[test]
    fn test_transpose_matrix() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12]];
        let transposed_matrix = transpose_matrix(&matrix);
        let expected_matrix = vec![vec![1, 4, 7, 10], vec![2, 5, 8, 11], vec![3, 6, 9, 12]];
        assert_eq!(expected_matrix, transposed_matrix);
    }

    #[test]
    fn test_generate_random_bit_vector() {
        let n = 8;
        let bit_vector = generate_random_bit_vector(n);
        for element in bit_vector {
            assert!(element == 0 || element == 1);
        }
    }

    #[test]
    fn test_matrix_mul_vector() {
        let q = 11;
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12]];
        let vector = vec![1, 2, 3];
        let resulting_vec = matrix_mul_vector(&matrix, &vector, q);
        let expected_vec = vec![3, 10, 6, 2];
        assert_eq!(expected_vec, resulting_vec);
        let matrix = vec![vec![2, 5, 1, 9], vec![7, 4, 8, 0], vec![6, 3, 2, 10]];
        let vector = vec![3, 7, 1, 4];
        let resulting_vec = matrix_mul_vector(&matrix, &vector, q);
        let expected_vec = vec![1, 2, 4];
        assert_eq!(expected_vec, resulting_vec);
    }

    #[test]
    fn test_inner_product_and_add() {
        let vec_1 = vec![1, 2, 3];
        let vec_2 = vec![4, 5, 6];
        let s = 1;
        let q = 11;
        let result = inner_product_and_add(&vec_1, &vec_2, s, q);
        let expected_result = 2;
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_mod_inv() {
        let a = 7;
        let m = 5;
        let inv = mod_inv(a, m).unwrap();
        assert_eq!(inv, 3);
    }
}