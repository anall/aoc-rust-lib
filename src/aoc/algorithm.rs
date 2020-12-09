use std::ops::{AddAssign, Sub};
use std::hash::Hash;
use std::collections::HashMap;

/// Determine the start and endpoint of a consecutive subsequence, where the subsequence sums to the given goal
///
/// The result is returned as [i,j)
pub fn consecutive_subsequence_with_sum<T: Default + AddAssign + Sub<Output = T> + Eq + Hash + Copy>(goal : T, data: &[T]) -> Option<(usize, usize)> {
    let mut rolling : T = Default::default();
    let mut table : HashMap<T,usize> = HashMap::with_capacity( data.len() + 1 );
    table.insert(rolling,0);
    for (j,val) in data.iter().enumerate() {
        rolling += *val;
        table.insert(rolling,j+1);
        let remain = rolling - goal;
        if let Some(i) = table.get(&remain) {
            return Some((*i, j+1));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::aoc::algorithm::consecutive_subsequence_with_sum;

    #[test]
    fn csws() {
        let data = vec![6,7,4,3,5,9,1,8];
        assert_eq!( consecutive_subsequence_with_sum(6,&data), Some((0,1)) );

        {
            let goal = 22;
            let (i,j) = consecutive_subsequence_with_sum(goal,&data).unwrap();
            assert_eq!( data[i..j].iter().sum::<i32>(), goal );
            assert_eq!( i, 2 );
            assert_eq!( j, 7 );
        }

        assert_eq!( consecutive_subsequence_with_sum(5,&data), Some((4,5)) );
        assert_eq!( consecutive_subsequence_with_sum(-16,&data), None );
    }

}

