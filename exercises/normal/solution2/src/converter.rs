pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
    let parts:Vec<&str> = num_str.split(&['(',')'][..]).collect();

    let base :u32= parts[1].parse().unwrap();
    let decimal = i64::from_str_radix(parts[0], base).unwrap();
    
    let res = if decimal == 0 {
        "0".to_string()
    } else {
        let mut num = decimal;
        let mut res = String::new();
        while num > 0 {
            let rem = num % to_base as i64;
            let digit = if rem < 10 {
                (rem as u8 + b'0') as char
            } else {
                (rem as u8 - 10 + b'a') as char
            };
            res.push(digit);
            num /= to_base as i64;
        }
        res.chars().rev().collect()
    };
    res
}
