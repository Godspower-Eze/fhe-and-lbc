////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
//////////////////////// RESIDUE NUMBER SYSTEM ////////////////////////////
///////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////

use crate::utils::{mod_inv};

fn construct(x: i128, q: &[i128]) -> (Vec<i128>, Vec<i128>){
    let r: Vec<i128> = q.iter().map(|&q_i| x.rem_euclid(q_i)).collect();
    (r, q.to_vec())
}

fn add_res(x: &(Vec<i128>, Vec<i128>), y: &(Vec<i128>, Vec<i128>)) -> (Vec<i128>, Vec<i128>) {
    debug_assert_eq!(x.1, y.1);
    let mut r = vec![];
    let length = x.0.len();
    for i in 0..length {
        r.push((x.0[i] + y.0[i]).rem_euclid(x.1[i]));
    }
    (r, x.1.clone())
}

fn sub_res(x: &(Vec<i128>, Vec<i128>), y: &(Vec<i128>, Vec<i128>)) -> (Vec<i128>, Vec<i128>) {
    debug_assert_eq!(x.1, y.1);
    let mut r = vec![];
    let length = x.0.len();
    for i in 0..length {
        r.push((x.0[i] - y.0[i]).rem_euclid(x.1[i]));
    }
    (r, x.1.clone())
}

fn mul_res(x: &(Vec<i128>, Vec<i128>), y: &(Vec<i128>, Vec<i128>)) -> (Vec<i128>, Vec<i128>) {
    debug_assert_eq!(x.1, y.1);
    let mut r = vec![];
    let length = x.0.len();
    for i in 0..length {
        r.push((x.0[i] * y.0[i]).rem_euclid(x.1[i]));
    }
    (r, x.1.clone())
}

fn deconstruct(res: (Vec<i128>, Vec<i128>)) -> i128 {
    let length = res.0.len();
    let Q = res.1.iter().copied().reduce(|acc, e| acc * e).unwrap();
    println!("{}", Q);
    let Q_i: Vec<i128> = res.1.iter().map(|q| Q / q).collect();
    let M_i: Vec<i128> = Q_i.iter().zip(res.1).map(|(Q_i, q_i)| mod_inv(*Q_i, q_i).unwrap()).collect();
    let mut x = 0;
    for i in 0..length {
        let v = res.0[i] * Q_i[i] * M_i[i];
        x += v % Q
    }
    x = x % Q;
    x
}


#[cfg(test)]
mod test {
    use rand::Rng;
    use crate::{rns::{add_res, construct, deconstruct, mul_res, sub_res}, utils::generate_primes};

    #[test]
    fn test_construct_and_deconstruct() {
        let rounds = 100;
        let mut rng = rand::rng();
        for _ in 0..rounds {
            // Generate a random 5-digit number
            let x: i128 = rng.random_range(10000..100000);

            let k = 20;
            let q = generate_primes(k);

            let x_i = construct(x, &q);

            let actual = deconstruct(x_i);
            assert_eq!(x, actual);
        }
    }

    #[test]
    fn test_add() {
        let rounds = 100;
        let mut rng = rand::rng();
        for _ in 0..rounds {
            // Generate a random 5-digit number
            let x: i128 = rng.random_range(10000..100000);
            let y: i128 = rng.random_range(10000..100000);

            let k = 20;
            let q = generate_primes(k);

            let x_i = construct(x, &q);
            let y_i = construct(y, &q);

            let res_from_add = add_res(&x_i, &y_i);
            let result = deconstruct(res_from_add);

            let Q = q.iter().copied().reduce(|acc, e| acc * e).unwrap();
            let actual = (x + y).rem_euclid(Q);

            assert_eq!(result, actual);
        }
    }


    #[test]
    fn test_sub() {
        let rounds = 100;
        let mut rng = rand::rng();
        for _ in 0..rounds {
            // Generate a random 5-digit number
            let x: i128 = rng.random_range(10000..100000);
            let y: i128 = rng.random_range(10000..100000);

            let k = 20;
            let q = generate_primes(k);

            let x_i = construct(x, &q);
            let y_i = construct(y, &q);

            let res_from_sub = sub_res(&x_i, &y_i);
            let result = deconstruct(res_from_sub);

            let Q = q.iter().copied().reduce(|acc, e| acc * e).unwrap();
            let actual = (x - y).rem_euclid(Q);

            assert_eq!(result, actual);
        }
    }


    #[test]
    fn test_mul() {
        let rounds = 100;
        let mut rng = rand::rng();
        for _ in 0..rounds {
            // Generate a random 5-digit number
            let x: i128 = rng.random_range(10000..100000);
            let y: i128 = rng.random_range(10000..100000);

            let k = 20;
            let q = generate_primes(k);

            let x_i = construct(x, &q);
            let y_i = construct(y, &q);

            let res_from_mul = mul_res(&x_i, &y_i);
            let result = deconstruct(res_from_mul);

            let Q = q.iter().copied().reduce(|acc, e| acc * e).unwrap();
            let actual = (x * y).rem_euclid(Q);

            assert_eq!(result, actual);
        }
    }

}