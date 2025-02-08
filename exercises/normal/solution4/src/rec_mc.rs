pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    let pays = &[1,2,5,10,20,30,50,100];
    let mut rem = amount;
    let mut cnt = 0;
    for &coin in pays.iter().rev() {
        while rem  >= coin {
            rem -= coin;
            cnt += 1
        }
    }

    cnt
}
