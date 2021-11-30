use crate::collections::base_algo::create_order;
use crate::io::input::Input;
use crate::io::output::{Output, Writable};
use std::ops::Mul;

#[derive(Clone)]
pub struct Permutation {
    p: Vec<usize>,
    base: usize,
}

impl Permutation {
    pub fn new(p: Vec<usize>) -> Self {
        Self::new_with_base(p, 0)
    }

    pub fn new_with_base(mut p: Vec<usize>, base: usize) -> Self {
        if base != 0 {
            p.iter_mut().for_each(|a| *a -= base);
        }
        debug_assert!(Self::check(&p));
        Self { p, base }
    }

    pub fn new_ident(size: usize) -> Self {
        Self::new_ident_with_base(size, 0)
    }

    pub fn new_ident_with_base(size: usize, base: usize) -> Self {
        Self {
            p: create_order(size),
            base,
        }
    }

    pub fn inv(&self) -> Self {
        let size = self.p.len();
        let mut res = Vec::with_capacity(size);
        unsafe {
            res.set_len(size);
        }
        for i in 0..size {
            res[self.p[i]] = i;
        }
        Self {
            p: res,
            base: self.base,
        }
    }

    fn check(p: &Vec<usize>) -> bool {
        let mut was = vec![false; p.len()];
        for i in p {
            if was[*i] {
                return false;
            }
            was[*i] = true;
        }
        true
    }
}

pub trait PermutationRead {
    fn read_permutation(&mut self, size: usize) -> Permutation {
        self.read_permutation_with_base(size, 1)
    }
    fn read_permutation_with_base(&mut self, size: usize, base: usize) -> Permutation;
}

impl PermutationRead for Input<'_> {
    fn read_permutation_with_base(&mut self, size: usize, base: usize) -> Permutation {
        Permutation::new_with_base(self.read_vec(size), base)
    }
}

impl Writable for Permutation {
    fn write(&self, output: &mut Output) {
        output.print_iter(self.p.iter().map(|i| *i + self.base));
    }
}

impl Mul for &Permutation {
    type Output = Permutation;

    fn mul(self, rhs: Self) -> Self::Output {
        let size = self.p.len();
        let mut res = Vec::with_capacity(size);
        unsafe {
            res.set_len(size);
        }
        for i in 0..size {
            res[i] = self.p[rhs.p[i]];
        }
        Permutation {
            p: res,
            base: self.base,
        }
    }
}