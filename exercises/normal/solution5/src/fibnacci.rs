pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // TODO: 这里写逻辑
    let (mut cur, mut next) = (0,1);
    let mut sum = 0;

    while cur < threshold {
        if cur %2 != 0 {
            sum +=cur;
            dbg!(sum);
        }
        let new = cur + next;
        cur = next;
        next = new;
    }
    
    sum
}
