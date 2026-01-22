////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
///// PUBLIC KEY CRYPTOGRAPHY WITH LEARNING WITH ERROR (LWE) PROBLEM //////
///////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////

use crate::utils::{add_vec, center_mod, inner_product_and_add, matrix_mul_vector, transpose_matrix};

pub fn generate_public_key(a: &Vec<Vec<i128>>, secret_key: &Vec<i128>, sampled_error: &Vec<i128>, q:i128) -> (Vec<Vec<i128>>, Vec<i128>) {
    // Compute b = As + e
    debug_assert!(a[0].len() == secret_key.len(), "column of a should be equal to length of error vector");
    let As = matrix_mul_vector(&a, &secret_key, q);
    let b = add_vec(&As, &sampled_error, q);
    (a.to_vec(), b)
}

pub fn encrypt(public_key: &(Vec<Vec<i128>>, Vec<i128>), r: &Vec<i128>, message: u8, q: i128) -> (Vec<i128>, i128) {
    let a_t = transpose_matrix(&public_key.0); // A transpose
    let u = matrix_mul_vector(&a_t, &r, q); // u = (A^T)r
    let v = (inner_product_and_add(&public_key.1, &r, 0, q) + ((message as i128) * (q / 2))).rem_euclid(q);
    (u, v)
}
 
fn decrypt(ciphertext: &(Vec<i128>, i128), secret_key: &Vec<i128>, q: i128) -> i128 {
    let u_s = inner_product_and_add(&ciphertext.0, &secret_key, 0, q);
    let result = (ciphertext.1 - u_s).rem_euclid(q);
    let encoded_message = center_mod(result, q);
    let mid_point = q / 2;
    if encoded_message.abs() <= (mid_point / 2) {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod test {
    use crate::{pub_lwe::{decrypt, encrypt, generate_public_key}, utils::{generate_random_bit_vector, generate_random_matrix, generate_random_vector, sample_discrete_gaussian_vector}};

    #[test]
    fn test_generate_public_key() {
        let q = 11;
        let A = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let s = vec![1, 2, 3];
        let e = vec![1, -1, -1];
        let public_key = generate_public_key(&A, &s, &e, q);
        let expected_value = (A, vec![4, 9, 5]);
        assert_eq!(public_key, expected_value);
    }

    #[test]
    fn test_encrypt() {
        let q = 11;
        let A = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let s = vec![1, 2, 3];
        let e = vec![1, -1, -1];
        let public_key = generate_public_key(&A, &s, &e, q);
        let bit_vector = vec![1, 0, 1];
        let message = 1;
        let encryption = encrypt(&public_key,&bit_vector, message as u8, q);
        let expected_value = (vec![8, 10, 1], 3);
        assert_eq!(encryption, expected_value);
    }

    #[test]
    fn test_decrypt() {
        let q = 11;
        let s = vec![1, 2, 3];
        let encryption = (vec![8, 10, 1], 3);
        let decryption = decrypt(&encryption, &s, q);
        assert_eq!(1, decryption);

        let encryption = (vec![8, 10, 1], 9);
        let decryption = decrypt(&encryption, &s, q);
        assert_eq!(0, decryption)

    }

    #[test]
    fn test_encrypt_and_decrypt() {
        let q = 17;
        let m = 4;
        let n = 4; 

        let rounds = 100;

        for _ in 0..rounds {
            let a = generate_random_matrix(m, n, q);
            let secret_key = generate_random_vector(n, q);
            let sigma: f64 = 0.5;
            let sampled_error = sample_discrete_gaussian_vector(sigma, m);
            let public_key = generate_public_key(&a, &secret_key, &sampled_error, q as i128);
            let r = generate_random_bit_vector(m);
            let message = generate_random_vector(n, 2)[0];
            let ciphertext = encrypt(&public_key, &r, message as u8, q as i128);

            let decryption = decrypt(&ciphertext, &secret_key, q as i128);
            assert_eq!(decryption, message)
        }
    }   
}