pub fn goldbach_conjecture() -> String {
    const MAX: usize = 10000;
    let mut prime = vec![false; MAX + 5];
    let mut sum = vec![false; MAX + 5];
    let mut primes = Vec::new();
    
    for i in 2..=MAX {
        if !prime[i] {
            primes.push(i);
            sum[i] = true;
        }
        for &p in &primes {
            if p * i > MAX {
                break;
            }
            prime[p * i] = true;
            if i % p == 0 {
                break;
            }
        }
    }

    for &p in &primes {
        for n in 1.. {
            let value = p + 2 * n * n;
            if value > MAX {
                break;
            }
            sum[value] = true;
        }
    }

    let result: Vec<String> = (11..MAX).step_by(2)
        .filter(|&i| !sum[i])
        .map(|i| i.to_string())
        .collect();

    // 返回用逗号隔开的字符串
    result.join(",")
}
