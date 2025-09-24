#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }

    if is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

fn is_sublist(sublist: &[i32], list: &[i32]) -> bool {
    if sublist.is_empty() {
        return true;
    }

    if sublist.len() > list.len() {
        return false;
    }

    // Check every possible starting position in the main list
    for start in 0..=(list.len() - sublist.len()) {
        if &list[start..start + sublist.len()] == sublist {
            return true;
        }
    }

    false
}
