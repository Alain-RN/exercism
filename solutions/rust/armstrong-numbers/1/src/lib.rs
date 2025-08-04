pub fn is_armstrong_number(num: u32) -> bool {
    let mut _num = num;
    let mut digits: Vec<u32> = Vec::new();
    while 0 < _num {
        let digit = (_num % 10) as u32;
        digits.push(digit);
        _num /= 10;
    }
    let mut sum = 0;
    let len_v: u32= digits.len() as u32;

    for d in digits {
         sum += d.pow(len_v);
    }
    sum as u32 == num
}
