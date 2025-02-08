pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    if n > 365 {
        return 1.0;
    }

    let mut prob = 1.0;
    let days = 365.0;

    for i in 0..n {
        prob *= (days - i as f64) / 365.0;
    }

    (1.0 - prob).mul_add(10_000.0, 0.5).floor() / 10_000.0
}
