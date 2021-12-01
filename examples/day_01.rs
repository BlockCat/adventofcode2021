#![feature(test)]
#![feature(portable_simd)]

use std::simd::u32x16;
extern crate test;

pub fn main() {
    let numbers: Vec<isize> = include_str!("../input/day01.txt")
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect();

    println!("{}", exercise::<2>(&numbers));
    println!("{}", exercise::<4>(&numbers));
}

// a + (b + c) > (b + c) + d => a > d
fn exercise<const N: usize>(count: &[isize]) -> usize {
    count
        .windows(N)
        .filter(|window| window[N - 1] > window[0])
        .count()
}


// #[cfg(test)]
mod benches {
    use crate::{*, test::Bencher};

    #[bench]
    fn input(b: &mut Bencher) {
        b.iter(|| {
            include_str!("../input/day01.txt")
                .lines()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        });
    }

    #[bench]
    fn ex1_normal(b: &mut Bencher) {
        let numbers: Vec<isize> = include_str!("../input/day01.txt")
            .lines()
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        b.iter(|| exercise::<2>(&numbers));
    }

    #[bench]
    fn ex2_normal(b: &mut Bencher) {
        let numbers: Vec<isize> = include_str!("../input/day01.txt")
            .lines()
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        b.iter(|| exercise::<4>(&numbers));
    }
}
