////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////
///// PUBLIC KEY CRYPTOGRAPHY WITH LEARNING WITH ERROR (LWE) Problem //////
/////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////

use rand::rngs::OsRng;
use rand::Rng;

const RangeOfError: [u8; 2] = [0, 1];

const RangeOfMessage: [u8; 2] = [0, 5];

fn add_vec(vec_1: &Vec<i128>, vec_2: &Vec<i128>, q: i128) -> Vec<i128> {
    let mut resulting_vec = vec![];
    for (e_1, e_2) in vec_1.iter().zip(vec_2) {
        let sum = (e_1 + e_2).rem_euclid(q);
        resulting_vec.push(sum);
    }
    resulting_vec
}

fn encrypt(q: i128, n: usize, threshold: usize, pairs: Vec<(Vec<i128>, i128)>, message: i128) -> (Vec<i128>, i128) {
    let mut choosen_pairs: Vec<(Vec<i128>, i128)> = vec![];
    while choosen_pairs.len() < threshold {
        let mut rng = OsRng;
        let i: usize = rng.gen_range(0..pairs.len());
        let random_public_key = pairs[i].clone();
        choosen_pairs.push(random_public_key);
    }
    let mut combined_pair = choosen_pairs.into_iter().reduce(|(acc_x, acc_y), (e_x, e_y)| (add_vec(&acc_x, &e_x, q), ((acc_y + e_y) % q) )).unwrap();
    let y = combined_pair.1 - message;
    combined_pair.1 = y;
    combined_pair
}

#[cfg(test)]
mod test {
    
    #[test]
    fn test_encrypt() {
        
    }
}