#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn sublist_of<T: PartialEq>(short_list: &[T], long_list: &[T]) -> Comparison {
    let short_len = short_list.len();
    let last_possible_index = long_list.len() - short_len;

    for (pos, item) in long_list[..=last_possible_index].iter().enumerate() {
        if short_list[0] == *item && short_list == &long_list[pos..(pos + short_len)] {
            return Comparison::Sublist;
        }
    }

    Comparison::Unequal
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let l1 = first_list.len();
    let l2 = second_list.len();

    match (l1, l2) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (_, _) if l1 == l2 => {
            if first_list == second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        (_, _) if l1 < l2 => sublist_of(first_list, second_list),
        (_, _) => {
            let r = sublist_of(second_list, first_list);
            if r == Comparison::Sublist {
                Comparison::Superlist
            } else {
                r
            }
        }
    }
}
