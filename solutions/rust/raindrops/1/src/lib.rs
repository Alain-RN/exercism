pub fn raindrops(n: u32) -> String {
    let mut _raindrop: Vec<&str> = Vec::new();

    if n % 3 != 0 && n % 5 !=0 && n % 7 !=0 {
        return n.to_string();
    }
    if n % 3 == 0 {
        _raindrop.push("Pling");
    }
    
    if n % 5 == 0 {
        _raindrop.push("Plang");
    }    
    
    if n % 7 == 0 {
        _raindrop.push("Plong");
    }


    _raindrop.join("")
}
