pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return ([]).to_vec();
    }
    let mut _garden : Vec<String> = Vec::new();
    let len_g = garden.len();
    let len_sub_g = garden[0].chars().count();
    
    for i in 0..len_g  {
        let mut _sub_garden = String::new();
        for j in 0..len_sub_g  {
            if garden[i].chars().nth(j).unwrap() == '*' {
                _sub_garden = format!("{}{}",_sub_garden, "*" );
            } else if nbr_flower_adj(garden, (i, j)) == 0 {
                    _sub_garden = format!("{}{}",_sub_garden, " " );
            } else {
                    _sub_garden = format!("{}{}",_sub_garden, nbr_flower_adj(garden, (i, j)))
            }
        }
        _garden.push(_sub_garden);
    }
    _garden
}


fn nbr_flower_adj( garden: &[&str], pos: (usize, usize)  ) -> u32 {
    let rows = garden.len();
    let cols = garden[0].chars().count();
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),          (0, 1),
        (1, -1),  (1, 0), (1, 1),
    ];

    let mut count = 0;

    for (dx, dy) in directions {
        let nx = pos.0 as isize + dx;
        let ny = pos.1 as isize + dy;

        if nx >= 0 && ny >= 0 && nx < rows as isize && ny < cols as isize {
            if garden[nx as usize].chars().nth(ny as usize).unwrap() == '*' {
                count += 1;
            }
        }
    }
    count
}

