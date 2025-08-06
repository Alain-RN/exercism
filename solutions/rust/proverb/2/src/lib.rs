pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
       return String::new();
    }
    if list.len() == 1 {
       return "And all for the want of a nail.".to_string();
    } 
    let mut _proverb: Vec<String> = Vec::new();
    let mut is_first: bool = true;
    let mut b = String::new();
    for s in list {
        if is_first {
            b = s.to_string();
            is_first = false;
        } else {
            _proverb.push(format!("For want of a {b} the {s} was lost."));
            b = s.to_string();
        }
    }
    _proverb.push(format!("And all for the want of a {}.", list[0]));
    _proverb.join("\n")
}
