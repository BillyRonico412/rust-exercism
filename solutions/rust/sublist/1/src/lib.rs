#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.len() == second_list.len() {
        for (i, n1) in first_list.iter().enumerate() {
            if *n1 != second_list[i] {
                return Comparison::Unequal;
            }
        }
        return Comparison::Equal;
    }
    if first_list.len() == 0 {
        return Comparison::Sublist;
    }
    if second_list.len() == 0 {
        return Comparison::Superlist;
    }
    let (super_list, sub_list, comp) = if first_list.len() >= second_list.len() {
        (first_list, second_list, Comparison::Superlist)
    } else {
        (second_list, first_list, Comparison::Sublist)
    };
    for (i, _) in super_list[..(super_list.len() - sub_list.len() + 1)]
        .iter()
        .enumerate()
    {
        if super_list[i..(i + sub_list.len() as usize)] == *sub_list {
            return comp;
        }
    }
    Comparison::Unequal
}
