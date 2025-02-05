use std::cmp::Ordering;

pub fn min<T: Ord>(left: T, right: T) -> T {
    match left.cmp(&right) {
        Ordering::Less | Ordering::Equal => left,
        Ordering::Greater => right,
    }

}
