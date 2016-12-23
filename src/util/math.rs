// Sum[i, {i, 1, n}]
pub fn sum(n: u32) -> u32 {
    n * (n + 1) / 2
}

// Sum[i^2, {i, 1, n}]
pub fn sum_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}


pub mod pythag {
    #[derive(PartialEq, PartialOrd, Ord, Debug, Eq, Clone)]
    pub struct Triple(pub u32, pub u32, pub u32);
    impl Triple {
        pub fn root() -> Triple {
            Triple(3, 4, 5)
        }
        pub fn new(a: u32, b: u32, c: u32) -> Triple {
            assert!(a < b && b < c && a * a + b * b == c * c);
            Triple(a, b, c)
        }
        pub fn check(a: u32, b: u32, c: u32) -> Option<Triple> {
            if a < b && b < c && a * a + b * b == c * c {
                Some(Triple(a, b, c))
            } else {
                None
            }
        }

        pub fn sum(&self) -> u32 {
            self.0 + self.1 + self.2
        }
        pub fn product(&self) -> u32 {
            self.0 * self.1 * self.2
        }
        fn scale(&mut self, scale_factor: u32) {
            self.0 *= scale_factor;
            self.1 *= scale_factor;
            self.2 *= scale_factor;
        }
        pub fn scaled(&self, scale_factor: u32) -> Triple {
            let mut t = self.clone();
            t.scale(scale_factor);
            t
        }
        pub fn scaled_triples(&self) -> ScaledTriples {
            ScaledTriples {
                primitive: self.clone(),
                scale_factor: 1,
            }
        }
        pub fn branch<'a>(&self) -> Branch {
            Branch {
                root: self.clone(),
                idx: 0,
            }
        }
    }

    pub struct ScaledTriples {
        primitive: Triple,
        scale_factor: u32,
    }

    impl Iterator for ScaledTriples {
        type Item = Triple;
        fn next(&mut self) -> Option<Triple> {
            let v = self.primitive.scaled(self.scale_factor);
            self.scale_factor += 1;
            Some(v)
        }
    }

    pub struct Branch {
        root: Triple,
        idx: usize
    }
    impl Iterator for Branch {
        type Item = Triple;
        fn next(&mut self) -> Option<Triple> {
            let Triple(a, b, c) = self.root;
            let t = match self.idx {
              0 => Triple(2 * c - 2 * a + b, 2 * c - a + 2 * b, 3 * c - 2 * a + 2 * b),
              1 => Triple(2 * c + 2 * a + b, 2 * c + a + 2 * b, 3 * c + 2 * a + 2 * b),
              2 => Triple(2 * c + a - 2 * b, 2 * c + 2 * a - b, 3 * c + 2 * a - 2 * b),
              _ => return None,
            };
            self.idx += 1;
            Some(t)
        }
    }
}
