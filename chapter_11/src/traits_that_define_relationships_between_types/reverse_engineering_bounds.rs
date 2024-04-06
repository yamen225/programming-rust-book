fn dot(v1: &[i64], v2: &[i64]) -> i64 {
    let mut total = 0;
    for i in 0 .. v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

/*

fn dot_gen<N>(v1: &[N], v2: &[N]) -> N {
    let mut total: N = 0; // error: mismatched types, zero is always an integer
    for i in 0 .. v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}
*/

use std::ops::{Add, Mul};
/*
fn dot_gen<N: Add + Mul + Default>(v1: &[N], v2: &[N]) -> N {
    let mut total = N::default(); // error: mismatched types, zero is always an integer
    for i in 0 .. v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

error: mismatched types
  |
5 | fn dot<N: Add + Mul + Default>(v1: &[N], v2: &[N]) -> N {
  |        - this type parameter
...
8 |         total = total + v1[i] * v2[i];
  |                         ^^^^^^^^^^^^^ expected type parameter `N`,
  |                                       found associated type
  |
  = note: expected type parameter `N`
            found associated type `<N as Mul>::Output`
help: consider further restricting this bound
  |
5 | fn dot<N: Add + Mul + Default + Mul<Output = N>>(v1: &[N], v2: &[N]) -> N {
  |                               ^^^^^^^^^^^^^^^^^
 */

 /*
 fn dot_gen<N>(v1: &[N], v2: &[N]) -> N
 where N: Add<Output = N> + Mul<Output = N> + Default
 {
     let mut total = N::default(); // error: mismatched types, zero is always an integer
     for i in 0 .. v1.len() {
         total = total + v1[i] * v2[i];
     }
     total
 }

 error: cannot move out of type `[N]`, a non-copy slice
  |
8 |         total = total + v1[i] * v2[i];
  |                         ^^^^^
  |                         |
  |                         cannot move out of here
  |                         move occurs because `v1[_]` has type `N`,
  |                         which does not implement the `Copy` trait
  */

fn dot_gen<N>(v1: &[N], v2: &[N]) -> N
 where N: Add<Output = N> + Mul<Output = N> + Default + Copy
 {
     let mut total = N::default(); // error: mismatched types, zero is always an integer
     for i in 0 .. v1.len() {
         total = total + v1[i] * v2[i];
     }
     total
 }

 #[cfg(test)]
 mod tests {
        use super::*;
    
        #[test]
        fn test_dot_gen() {
            let v1 = [1, 2, 3];
            let v2 = [4, 5, 6];
            assert_eq!(32, dot_gen(&v1, &v2));
        }
 }

 use num::Num;

 fn dot_gen_num<N: Num + Copy>(v1: &[N], v2: &&[N]) -> N {
    let mut total = N::zero();
    for i in 0 .. v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
 }