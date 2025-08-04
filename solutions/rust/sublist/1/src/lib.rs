#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    use crate::Comparison::*;

    if first_list == second_list {
        return Equal;
    }

    if first_list.len() == 0 {
        return Sublist;
    }

    if second_list.len() == 0 {
        return Superlist;
    }

    if first_list.len() <= second_list.len() {
        for i in 0..=(second_list.len() - first_list.len()) {
            if first_list == &second_list[i..(first_list.len() + i) ] {
                return Sublist;
            }
        } 
    } else {
        for i in 0..=(first_list.len() - second_list.len()) {
            if second_list == &first_list[i..(second_list.len() + i) ] {
                return Superlist;
            }
        } 
    }

    Unequal
}
