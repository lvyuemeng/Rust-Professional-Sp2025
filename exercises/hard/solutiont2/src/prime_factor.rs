fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

fn mod_mul(mut a: u128, mut b: u128, m: u128) -> u128 {
    let mut result = 0;
    a %= m;
    while b > 0 {
        if b & 1 == 1 {
            result = (result + a) % m;
        }
        a = (a << 1) % m;
        b >>= 1;
    }
    result
}

fn mod_pow(mut base: u128, mut exp: u128, m: u128) -> u128 {
    let mut result = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            result = mod_mul(result, base, m);
        }
        base = mod_mul(base, base, m);
        exp >>= 1;
    }
    result
}

/// Miller–Rabin primality test.
/// Test by a^(p-1) = 1 mod p for random a.
fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    // n − 1 = d · 2^s.
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    // random set for test 
    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    'outer: for &a in &bases {
        if a >= n {
            break;
        }
        let mut x = mod_pow(a, d, n);
        // early return
        if x == 1 || x == n - 1 {
            continue 'outer;
        }

        for _ in 0..(s - 1) {
            x = mod_mul(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }

        return false;
    }
    true
}

/// Pollard–Rho algorithm 
fn pollard_rho(n: u128) -> u128 {
    if n % 2 == 0 {
        return 2;
    }
    let mut x = 2;
    let mut y = 2;
    let mut c = 1;
    let mut d = 1;
    // generate random: x^2 + 1 mod N
    while d == 1 {
        x = (mod_mul(x, x, n) + c) % n;
        y = (mod_mul(y, y, n) + c) % n;
        y = (mod_mul(y, y, n) + c) % n;
        d = gcd(if x > y { x - y } else { y - x }, n);
        if d == n {
            // failed, try another
            c += 1;
            x = 2;
            y = 2;
            d = 1;
        }
    }
    d
}

fn factorize(n: u128, factors: &mut Vec<u128>) {
    if n == 1 {
        return;
    }
    if is_prime(n) {
        factors.push(n);
    } else {
        let factor = pollard_rho(n);
        // randomly find, then decompose.
        factorize(factor, factors);
        factorize(n / factor, factors);
    }
}

/// Returns the maximum prime factor of n.
pub fn find_max_prime_factor(n: u128) -> u128 {
    let mut factors = Vec::new();
    factorize(n, &mut factors);
    factors.into_iter().max().unwrap()
}