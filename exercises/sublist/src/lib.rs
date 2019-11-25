#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn sublist_of<T: PartialEq>(short_list: &[T], long_list: &[T]) -> bool {
    let short_len = short_list.len();

    short_len == 0 || long_list.windows(short_len).any(|w| w == short_list)
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use std::cmp::Ordering;
    use Comparison::*;

    let l1 = first_list.len();
    let l2 = second_list.len();

    match l1.cmp(&l2) {
        Ordering::Equal if first_list == second_list => Equal,
        Ordering::Less if sublist_of(first_list, second_list) => Sublist,
        Ordering::Greater if sublist_of(second_list, first_list) => Superlist,
        _ => Unequal,
    }
}
