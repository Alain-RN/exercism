pub fn square_of_sum(n: u32) -> u32 {
    let mut i = 1;
    let mut sum: u32 = 0;
    loop {
        sum += i;
        if i == n {
            break;
        }
        i+=1;
        
    }
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut i: u32 = 1;
    let mut sum: u32 = 0;
    loop {
        sum += i.pow(2);
        if i == n {
            break;
        }
        i+=1;
        
    }
    sum
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n) 
}
