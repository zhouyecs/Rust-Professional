pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    // todo!()
    if n > 365 {
        return 1.0;
    }
    
    let mut prob_diff = 1.0;
    for i in 0..n {
        prob_diff *= (365 - i) as f64 / 365.0;
    }
    
    1.0 - prob_diff
}
