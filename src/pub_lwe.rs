////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
///// PUBLIC KEY CRYPTOGRAPHY WITH LEARNING WITH ERROR (LWE) PROBLEM //////
///////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////

use rand::rngs::OsRng;
use rand::Rng;

fn add_vec(vec_1: &Vec<i128>, vec_2: &Vec<i128>, q: i128) -> Vec<i128> {
    let mut resulting_vec = vec![];
    for (e_1, e_2) in vec_1.iter().zip(vec_2) {
        let sum = (e_1 + e_2).rem_euclid(q);
        resulting_vec.push(sum);
    }
    resulting_vec
}

fn inner_product(vec_1: &Vec<i128>, vec_2: &Vec<i128>, q: i128) -> i128 {
    let mut result = 0;
    for (e_1, e_2) in vec_1.iter().zip(vec_2) {
        result += (e_1 * e_2).rem_euclid(q);
    }
    result.rem_euclid(q)
}

fn generate_random_vector(n:usize, q: usize) -> Vec<usize> {
    let mut secret_key = vec![];
    for _ in 0..n {
        let mut rng = OsRng;
        let val: usize = rng.gen_range(0..q);
        secret_key.push(val);
    }
    secret_key
}

fn generate_random_matrix(m: usize, n: usize, q: usize) -> Vec<Vec<usize>> {
    let mut matrix = vec![];
    for _ in 0..m {
        let row = generate_random_vector(n, q);
        matrix.push(row);
    }
    matrix
}

fn encrypt(q: i128, threshold: usize, public_keys: Vec<(Vec<i128>, i128)>, message: i128) -> (Vec<i128>, i128) {
    let mut choosen_public_keys: Vec<(Vec<i128>, i128)> = vec![];
    while choosen_public_keys.len() < threshold {
        let mut rng = OsRng;
        let i: usize = rng.gen_range(0..public_keys.len());
        let random_public_key = public_keys[i].clone();
        choosen_public_keys.push(random_public_key);
    }
    let mut combined_public_key = choosen_public_keys.into_iter().reduce(|(acc_x, acc_y), (e_x, e_y)| (add_vec(&acc_x, &e_x, q), ((acc_y + e_y).rem_euclid(q)) )).unwrap();
    let y = (combined_public_key.1 - message).rem_euclid(q);
    combined_public_key.1 = y;
    combined_public_key
}
 
fn decrypt(public_key: (Vec<i128>, i128), private_key: Vec<i128>, q: i128) -> i128 {
    // x.a + e = y + m
    let x_a = inner_product(&public_key.0, &private_key, q); // x.a
}

#[cfg(test)]
mod test {
    
    #[test]
    fn test_encrypt() {
        
    }

    #[test]
    fn test_decrypt() {
        
    }

    #[test]
    fn test_encrypt_and_decrypt() {
        
    }
}