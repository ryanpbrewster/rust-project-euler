// It turns out that 12 cm is the smallest length of wire that can be bent to
// form an integer sided right angle triangle in exactly one way, but there are
// many more examples.
//   12 cm: (3,4,5)
//   24 cm: (6,8,10)
//   30 cm: (5,12,13)
//   36 cm: (9,12,15)
//   40 cm: (8,15,17)
//   48 cm: (12,16,20)
// In contrast, some lengths of wire, like 20 cm, cannot be bent to form an
// integer sided right angle triangle, and other lengths allow more than one
// solution to be found; for example, using 120 cm it is possible to form
// exactly three different integer sided right angle triangles.
//   120 cm: (30,40,50), (20,48,52), (24,45,51)
// Given that L is the length of the wire, for how many values of L ≤ 1,500,000
// can exactly one integer sided right angle triangle be formed?

use crate::util::math::pythag;
use std::collections::VecDeque;

pub fn solve(bound: usize) -> usize {
    let mut xs = vec![0; bound];
    let mut q: VecDeque<pythag::Triple> = VecDeque::new();
    q.push_back(pythag::Triple::root());
    while let Some(t0) = q.pop_front() {
        let perimeter = t0.sum() as usize;
        if perimeter < bound {
            for p in (1..).map(|k| k * perimeter).take_while(|&p| p < bound) {
                xs[p] += 1;
            }
            for t in t0.branch() {
                q.push_back(t);
            }
        }
    }

    xs.iter().filter(|&&c| c == 1).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small() {
        assert_eq!(solve(100), 11);
    }

    #[test]
    fn main() {
        assert_eq!(solve(1_500_001), 161667);
    }
}
