#![feature(plugin)]
#![plugin(quickcheck_macros)]

pub mod stable;
pub mod unstable;

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod tests;
