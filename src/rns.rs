use crate::utils::{mod_inv, generate_primes};

fn construct(x: i128, q: &[i128]) -> Vec<i128>{
    q.iter().map(|&q_i| x.rem_euclid(q_i)).collect()
}

fn deconstruct(res: &[i128], moduli: &[i128]) -> i128 {
    let length = res.len();
    let Q = moduli.iter().copied().reduce(|acc, e| acc * e).unwrap();
    println!("{}", Q);
    let Q_i: Vec<i128> = moduli.iter().map(|q| Q / q).collect();
    let M_i: Vec<i128> = Q_i.iter().zip(moduli).map(|(Q_i, q_i)| mod_inv(*Q_i, *q_i).unwrap()).collect();
    let mut x = 0;
    for i in 0..length {
        let v = res[i] * Q_i[i] * M_i[i];
        x += v % Q
    }
    x = x % Q;
    x
}


#[cfg(test)]
mod test {
    use rand::Rng;
    use crate::{rns::{construct, deconstruct}, utils::generate_primes};

    #[test]
    fn construct_and_deconstruct() {
    let rounds = 100;
    let mut rng = rand::rng();

    for _ in 0..rounds {
        // Generate a random 5-digit number
        let x: i128 = rng.random_range(10000..100000);

        let k = 20;
        let q = generate_primes(k);

        let x_i = construct(x, &q);

        let actual = deconstruct(&x_i, &q);
        assert_eq!(x, actual);
    }
}
}