#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist_<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    for i in 0..=_second_list.len() {
        for j in i..=_second_list.len() {
            println!("testing i {}, {}  ", i, j);
            if _first_list[..] == _second_list[i..j] {
                println!("match");
                return true;
            }
        }
    }
    false
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let a_in_b = sublist_(_first_list, _second_list);
    let b_in_a = sublist_(_second_list, _first_list);

    match (a_in_b, b_in_a) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
