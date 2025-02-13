pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    // todo!()
    let mut dp = vec![0; (amount + 1) as usize];
    let cashes = [1, 2, 5, 10, 20, 30, 50, 100];
    for i in 1..=amount {
        dp[i as usize] = i;
        for &cash in cashes.iter() {
            if i >= cash {
                dp[i as usize] = dp[i as usize].min(dp[(i - cash) as usize] + 1);
            }
        }
    }
    dp[amount as usize]
}
