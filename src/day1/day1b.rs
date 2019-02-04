use std::collections::HashSet;
use crate::utils::IterableInput;

pub fn first_recurrence(iterable: &IterableInput) -> i32 {
    let mut sum = 0;
    let mut set = HashSet::new();

    set.insert(sum);
    loop {
        let it = iterable.iter_i32();
        for i in it {
            sum += i;
            if set.contains(&sum) {
                return sum;
            }
            set.insert(sum);
        }
    }
}
