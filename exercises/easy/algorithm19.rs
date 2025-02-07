/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number.
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.

    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

pub fn fib(n: i32) -> i32 {
    // TODO: Implement the logic to calculate the nth Fibonacci number using matrix exponentiation
    /*
    let mut  cur = 0;
    let mut next = 1;
    for _ in 0..n{
        let now = cur;
        cur = next;
        next = now + next;
    }
    cur*/
    if n == 0 {
        return 0;
    }
    let mat = vec![vec![1, 1], vec![1, 0]];

    let res = matrix_pow(&mat, n-1);

    let fib_n = res[0][0];
    fib_n
}

pub fn matrix_mul(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let len = a.len();
    let mut res = vec![vec![0;len]; len];
    for i in 0..len {
        for j in 0..len {
            for k in 0..len {
                res[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    res
}

pub fn matrix_pow(mat: &Vec<Vec<i32>>, n: i32) -> Vec<Vec<i32>> {
    if n== 0 {
        return vec![vec![1,0],vec![0,1]];
    }
    if n == 1 {
        return mat.clone();
    }

    if n % 2 == 0 {
        let half = matrix_pow(mat, n / 2);
        return matrix_mul(&half, &half);
    } else {
        let half_pow = matrix_pow(mat, n / 2);
        return matrix_mul(&matrix_mul(&half_pow, &half_pow), mat);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
