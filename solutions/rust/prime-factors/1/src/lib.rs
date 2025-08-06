pub fn factors(n: u64) -> Vec<u64> {
    let mut fact: Vec<u64> = Vec::new();
    let mut _n = n;
    let mut div = 2;
    while div <= _n {
        if _n % div == 0 {
            fact.push(div);
            _n /= div;
        } else {
            div += 1;
        }
    }
    fact
}
