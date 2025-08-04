pub fn nth(n: u32) -> u32 {
    let mut i = 0 ;
    let mut prime: u32 = 2;
    loop {
        if is_prime(prime) {
            i+=1;
        } 
        if i == n + 1 {
            return prime
        }
        prime += 1;
        
    }
    return prime;
}

fn is_prime(digit: u32) -> bool {
    if digit < 2 {
        return false;
    }
    for i in 2..=((digit as f64).sqrt() as u32) {
        if digit % i == 0 {
            return false;
        }
    }
    true
}