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

pub fn generate_random_vector(n:usize, q: usize) -> Vec<usize> {
    let mut secret_key: Vec<usize> = vec![];
    let mut rng = [0u8; 16];
    for _ in 0..n {
        OsRng.try_fill_bytes(&mut rng).unwrap();
        let random_val = OsRng.try_next_u64().unwrap();
        secret_key.push(random_val.try_into().unwrap());
    }
    secret_key
}

pub fn generate_random_matrix(m: usize, n: usize, q: usize) -> Vec<Vec<usize>> {
    let mut matrix = vec![];
    for _ in 0..m {
        let row = generate_random_vector(n, q);
        matrix.push(row);
    }
    matrix
}

pub fn sample_discrete_gaussian(sigma: f64) -> i128 {
    let mut rng = rand::rng();
    let normal: f64 = rng.sample(Normal::new(0.0, sigma).unwrap());
    normal.round() as i128
}

pub fn sample_discrete_gaussian_vector(sigma: f64, n: usize) -> Vec<i128> {
    (0..n).map(|_| sample_discrete_gaussian(sigma).try_into().unwrap()).collect()
} 