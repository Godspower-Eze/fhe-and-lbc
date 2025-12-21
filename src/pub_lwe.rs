////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
///// PUBLIC KEY CRYPTOGRAPHY WITH LEARNING WITH ERROR (LWE) PROBLEM //////
///////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////

use crate::utils::{inner_product_and_add};

fn generate_public_key(a: Vec<Vec<i128>>, secret_key: Vec<i128>, sampled_error: Vec<i128>, q:i128) -> Vec<(Vec<i128>, i128)>{
    // Compute b = As + e
    debug_assert!(a[0].len() == secret_key.len() && secret_key.len() == sampled_error.len());
    let mut public_key: Vec<(Vec<i128>, i128)> = vec![];
    for (i, row) in a.iter().enumerate() {
        let b = inner_product_and_add(row, &secret_key, sampled_error[i], q);
        public_key.push((row.to_vec(), b));
    }
    public_key
}

fn encrypt(public_key_: Vec<(Vec<i128>, i128)>, message: u8) {
    
}
 
// fn decrypt(public_key: (Vec<i128>, i128), private_key: Vec<i128>, q: i128) -> i128 {
//     // x.a + e = y + m
//     let x_a = inner_product(&public_key.0, &private_key, q); // x.a
// }

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