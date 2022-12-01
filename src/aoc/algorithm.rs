#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use std::cmp;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::{Enumerate, Fuse, FusedIterator};
use std::ops::{AddAssign, Sub};

pub trait IterAlgorithms: Iterator {
    /// Determine the start and endpoint of the first consecutive subsequence, where the subsequence sums to the given goal
    ///
    /// Each valid subsequence is returned as [i,j)
    fn consecutive_subsequences_with_sum<
        'a,
        I: 'a + Default + AddAssign + Sub<Output = I> + Eq + Hash + Copy + Clone,
    >(
        self,
        goal: I,
    ) -> ConsecutiveSubsequencesSum<'a, Self, I>
    where
        Self: Sized + Iterator<Item = &'a I>,
    {
        if let (_, Some(hint)) = self.size_hint() {
            ConsecutiveSubsequencesSum::new_with_size_hint(self, goal, cmp::max(hint, 4096))
        } else {
            ConsecutiveSubsequencesSum::new(self, goal)
        }
    }

    /// Determine the start and endpoint of the first consecutive subsequence, where the subsequence sums to the given goal
    ///
    /// The first subsequence is returned as [i,j)
    fn first_consecutive_subsequence_with_sum<
        'a,
        I: 'a + Default + AddAssign + Sub<Output = I> + Eq + Hash + Copy + Clone,
    >(
        self,
        goal: I,
    ) -> Option<(usize, usize)>
    where
        Self: Sized + Iterator<Item = &'a I>,
    {
        self.consecutive_subsequences_with_sum(goal).next()
    }
}
impl<T> IterAlgorithms for T where T: Iterator {}

pub struct ConsecutiveSubsequencesSum<
    'a,
    T,
    I: 'a + Default + AddAssign + Sub<Output = I> + Eq + Hash + Copy + Clone,
> where
    T: Iterator<Item = &'a I>,
{
    iter: Fuse<Enumerate<T>>,
    goal: I,
    prefix_sum: I,
    table: HashMap<I, usize>,
}
impl<'a, T, I: 'a + Default + AddAssign + Sub<Output = I> + Eq + Hash + Copy + Clone>
    ConsecutiveSubsequencesSum<'a, T, I>
where
    T: Iterator<Item = &'a I>,
{
    fn new(iter: T, goal: I) -> Self {
        let mut table = HashMap::new();
        table.insert(Default::default(), 0);
        Self {
            iter: iter.enumerate().fuse(),
            goal,
            prefix_sum: Default::default(),
            table,
        }
    }

    fn new_with_size_hint(iter: T, goal: I, hint: usize) -> Self {
        let mut table = HashMap::with_capacity(hint);
        table.insert(Default::default(), 0);
        Self {
            iter: iter.enumerate().fuse(),
            goal,
            prefix_sum: Default::default(),
            table,
        }
    }
}
impl<'a, T, I: 'a + Default + AddAssign + Sub<Output = I> + Eq + Hash + Copy + Clone> Iterator
    for ConsecutiveSubsequencesSum<'a, T, I>
where
    T: Iterator<Item = &'a I>,
{
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        for (j, val) in self.iter.by_ref() {
            self.prefix_sum += *val;
            self.table.insert(self.prefix_sum, j + 1);
            let remain = self.prefix_sum - self.goal;
            if let Some(i) = self.table.get(&remain) {
                return Some((*i, j + 1));
            }
        }
        None
    }
}
impl<'a, T, I: 'a + Default + AddAssign + Sub<Output = I> + Eq + Hash + Copy + Clone> FusedIterator
    for ConsecutiveSubsequencesSum<'a, T, I>
where
    T: Iterator<Item = &'a I>,
{
}

#[derive(Debug)]
pub struct Cycle<T> {
    pub cycle_at: T,
    pub n: usize,
    pub prev_n: usize,
}
pub fn find_cycle<T: Hash + Eq + Clone>(mut current: T, advance: impl Fn(T) -> T) -> Cycle<T> {
    let mut seen: HashMap<T, usize> = HashMap::new();
    let mut n: usize = 0;
    while !seen.contains_key(&current) {
        seen.insert(current.clone(), n);
        current = advance(current);
        n += 1;
    }
    let prev_n = *seen.get(&current).unwrap();
    Cycle {
        cycle_at: current,
        n,
        prev_n,
    }
}

pub fn find_cycle_iter<T: Hash + Eq + Clone, I: Iterator<Item = T>>(
    mut iter: I,
) -> Option<Cycle<T>> {
    let mut seen: HashMap<T, usize> = HashMap::new();
    let mut n: usize = 0;
    let mut current: T = iter.next()?;
    while !seen.contains_key(&current) {
        seen.insert(current.clone(), n);
        current = iter.next()?;
        n += 1;
    }
    let prev_n = *seen.get(&current).unwrap();
    Some(Cycle {
        cycle_at: current,
        n,
        prev_n,
    })
}

#[cfg(test)]
mod tests {
    use crate::aoc::algorithm::{find_cycle, IterAlgorithms};

    #[test]
    fn csws() {
        let data = vec![6, 7, 4, 3, 5, 9, 1, 8];
        assert_eq!(
            data.iter().first_consecutive_subsequence_with_sum(6),
            Some((0, 1))
        );

        {
            let goal = 22;
            let (i, j) = data
                .iter()
                .consecutive_subsequences_with_sum(goal)
                .next()
                .unwrap();
            assert_eq!(data[i..j].iter().sum::<i32>(), goal);
            assert_eq!(i, 2);
            assert_eq!(j, 7);
        }

        assert_eq!(
            data.iter().consecutive_subsequences_with_sum(5).next(),
            Some((4, 5))
        );
        assert_eq!(
            data.iter().consecutive_subsequences_with_sum(-16).next(),
            None
        );

        data.iter().consecutive_subsequences_with_sum(5);
    }

    #[test]
    fn test_find_loop() {
        let cy = find_cycle(0, |n| (n + 1) % 6);
        assert_eq!(cy.cycle_at, 0);
        assert_eq!(cy.n, 6);
        assert_eq!(cy.prev_n, 0);
    }
}
