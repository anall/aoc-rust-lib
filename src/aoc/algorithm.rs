use std::ops::{AddAssign, Sub};
use std::hash::Hash;
use std::collections::HashMap;

pub fn consecutive_subsequence_with_sum<T: Default + AddAssign + Sub<Output = T> + Eq + Hash + Copy>(goal : T, data: &[T]) -> Option<(usize, usize)> {
    if goal == data[0] { // we need this special case
        return Some((0,0));
    }

    let mut rolling : T = Default::default();
    let mut table : HashMap<T,usize> = HashMap::with_capacity( data.len() );
    for (j,val) in data.iter().enumerate() {
        rolling += *val;
        table.insert(rolling,j);
        let remain = rolling - goal;
        if let Some(i) = table.get(&remain) {
            return Some((*i+1, j));
        }
    }
    None
}