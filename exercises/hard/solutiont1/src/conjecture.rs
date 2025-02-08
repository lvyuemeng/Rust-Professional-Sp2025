pub fn goldbach_conjecture() -> String {
    (9..)
        .step_by(2)
        .filter(|&n| !is_prime(n))
        .filter(|&n| !(1..=((n as f64 / 2.0).sqrt() as u64)).any(|b| is_prime(n - 2 * b * b)))
        .take(2)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as u64) {
        if num % i == 0 {
            return false;
        }
    }
    true
}
