pub fn reverse(input: &str) -> String {
    let mut output = String::from("");
    let inv = input.chars().count();
    for i in 0..inv {
        output.push(input.chars().nth(inv - 1 - i).unwrap());
    }
    output
}
