#![feature(test)]
#![feature(plugin)]
#![plugin(quickcheck_macros)]

pub mod stable;
pub mod unstable;

#[cfg(test)]
extern crate quickcheck;
extern crate test;
extern crate rand;
mod tests;
mod bench;
