use std::ops::{AddAssign, Sub};
use std::hash::Hash;
use std::collections::HashMap;
use std::iter::{Enumerate, Fuse};
use std::cmp;

pub trait IterAlgorithms : Iterator {
    /// Determine the start and endpoint of the first consecutive subsequence, where the subsequence sums to the given goal
    ///
    /// Each valid subsequence is returned as [i,j)
    fn consecutive_subsequences_with_sum<'a,I : 'a + Default + AddAssign + Sub<Output = I> + Eq + Hash + Copy + Clone>(self, goal : I) -> ConsecutiveSubsequencesSum<'a, Self, I> where Self: Sized + Iterator<Item = &'a I> {
        if let (_,Some(hint)) = self.size_hint() {
            ConsecutiveSubsequencesSum::new_with_size_hint(self,goal,cmp::max(hint,4096))
        } else {
            ConsecutiveSubsequencesSum::new(self, goal)
        }
    }

    /// Determine the start and endpoint of the first consecutive subsequence, where the subsequence sums to the given goal
    ///
    /// The first subsequence is returned as [i,j)
    fn first_consecutive_subsequence_with_sum<'a,I : 'a + Default + AddAssign + Sub<Output = I> + Eq + Hash + Copy + Clone>(self, goal : I) -> Option<(usize,usize)> where Self: Sized + Iterator<Item = &'a I> {
        self.consecutive_subsequences_with_sum(goal).next()
    }
}
impl<T> IterAlgorithms for T where T: Iterator {}


pub struct ConsecutiveSubsequencesSum<'a,T, I : 'a + Default + AddAssign + Sub<Output = I> + Eq + Hash + Copy + Clone> where T: Iterator<Item = &'a I>  {
    iter : Fuse<Enumerate<T>>,
    goal : I,
    rolling : I,
    table : HashMap<I,usize>,
}
impl<'a,T, I : 'a + Default + AddAssign + Sub<Output = I> + Eq + Hash + Copy + Clone> ConsecutiveSubsequencesSum<'a,T,I> where T: Iterator<Item = &'a I>  {
    fn new(iter : T, goal: I) -> Self {
        let mut table = HashMap::new();
        table.insert( Default::default(), 0 );
        Self{
            iter: iter.enumerate().fuse(), goal,
            rolling: Default::default(),
            table
        }
    }

    fn new_with_size_hint(iter : T, goal: I, hint: usize) -> Self {
        let mut table = HashMap::with_capacity(hint);
        table.insert( Default::default(), 0 );
        Self{
            iter: iter.enumerate().fuse(), goal,
            rolling: Default::default(),
            table
        }
    }
}
impl<'a,T, I : 'a + Default + AddAssign + Sub<Output = I> + Eq + Hash + Copy + Clone> Iterator for ConsecutiveSubsequencesSum<'a,T,I> where T: Iterator<Item = &'a I> {    type Item = (usize,usize);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((j,val)) = self.iter.next() {
            self.rolling += *val;
            self.table.insert(self.rolling,j+1);
            let remain = self.rolling - self.goal;
            if let Some(i) = self.table.get(&remain) {
                return Some((*i, j+1));
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use crate::aoc::algorithm::IterAlgorithms;

    #[test]
    fn csws() {
        let data = vec![6,7,4,3,5,9,1,8];
        assert_eq!(data.iter().first_consecutive_subsequence_with_sum(6), Some((0, 1)) );

        {
            let goal = 22;
            let (i,j) = data.iter().consecutive_subsequences_with_sum(goal).next().unwrap();
            assert_eq!( data[i..j].iter().sum::<i32>(), goal );
            assert_eq!( i, 2 );
            assert_eq!( j, 7 );
        }

        assert_eq!(data.iter().consecutive_subsequences_with_sum(5).next(), Some((4, 5)) );
        assert_eq!(data.iter().consecutive_subsequences_with_sum(-16).next(), None );

        data.iter().consecutive_subsequences_with_sum( 5);
    }

}

