////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
///// PUBLIC KEY CRYPTOGRAPHY WITH LEARNING WITH ERROR (LWE) PROBLEM //////
///////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////

use crate::utils::{add_vec, generate_random_bit_vector, inner_product_and_add, matrix_mul_vector, transpose_matrix};

pub fn generate_public_key(a: &Vec<Vec<i128>>, secret_key: &Vec<i128>, sampled_error: &Vec<i128>, q:i128) -> (Vec<Vec<i128>>, Vec<i128>) {
    // Compute b = As + e
    debug_assert!(a[0].len() == secret_key.len(), "column of a should be equal to length of error vector");
    let As = matrix_mul_vector(&a, &secret_key, q);
    let b = add_vec(&As, &sampled_error, q);
    (a.to_vec(), b)
}

pub fn encrypt(public_key: &(Vec<Vec<i128>>, Vec<i128>), message: u8, m:usize, q: i128) -> (Vec<i128>, i128) {
    let r = generate_random_bit_vector(m); // r
    let a_t = transpose_matrix(&public_key.0); // A transpose
    let u = matrix_mul_vector(&a_t, &r, q); // u = (A^T)r
    let v = (inner_product_and_add(&public_key.1, &r, 0, q) + ((message as i128) * (q / 2))).rem_euclid(q);
    (u, v)
}

fn center_mod(mut val: i128, q: i128) -> i128 {
    if val > q/2 {    // map large positives to negative
        val -= q;
    }
    val
}
 
fn decrypt(ciphertext: &(Vec<i128>, i128), secret_key: &Vec<i128>, q: i128) -> i128 {
    let u_s = inner_product_and_add(&ciphertext.0, &secret_key, 0, q);
    let result = (ciphertext.1 - u_s).rem_euclid(q);
    let encoded_message = center_mod(result, q);
    let mid_point = q/2;
    if encoded_message < mid_point {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod test {
    use crate::{pub_lwe::{decrypt, encrypt, generate_public_key}, utils::{generate_random_matrix, generate_random_vector, sample_discrete_gaussian_vector}};


    #[test]
    fn test_encrypt_and_decrypt() {
        let q = 11;
        let m = 8;
        let n = 4; 
        
        let a = generate_random_matrix(m, n, q);
        let secret_key = generate_random_vector(n, q);
        let sigma: f64 = q as f64 / 4.0; // q/2/2
        let sampled_error = sample_discrete_gaussian_vector(sigma, m);
        let public_key = generate_public_key(&a, &secret_key, &sampled_error, q as i128);
        let message = 1;
        let ciphertext = encrypt(&public_key, message, m, q as i128);

        let message = decrypt(&ciphertext, &secret_key, q as i128);
        println!("{:?}", message);
    }   
}