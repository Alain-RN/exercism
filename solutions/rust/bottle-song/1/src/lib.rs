pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut r = String::new();
    for i in 0..take_down {
        r = format!("{}{}\n", r, one_part(start_bottles - i));
    }
    r
}

fn digit_string_bottles( digit: u32 )-> String {
    let text = match digit {
        1 => "One green bottle",
        2 => "Two green bottles",
        3 => "Three green bottles",
        4 => "Four green bottles",
        5 => "Five green bottles",
        6 => "Six green bottles",
        7 => "Seven green bottles",
        8 => "Eight green bottles",
        9 => "Nine green bottles",
        10 =>"Ten green bottles",
        _ => &digit_string_bottles(1),
    };
    text.to_string()
}

fn one_part(digit: u32) -> String {
    let l1_2 = format!("{} hanging on the wall,\n",digit_string_bottles(digit), );
    let l4 = if digit != 1 {
        format!( "There'll be {} hanging on the wall.\n",
                 digit_string_bottles(digit - 1).to_lowercase() )
    } else {
        String::from("There'll be no green bottles hanging on the wall.")
    };
    format!("{}{}And if one green bottle should accidentally fall,\n{}", l1_2, l1_2,l4)
}
