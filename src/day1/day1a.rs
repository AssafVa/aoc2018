use crate::utils::IterableInput;

pub fn sum_list(iterable: &IterableInput) -> i32 {
    iterable.iter_i32().sum()
}