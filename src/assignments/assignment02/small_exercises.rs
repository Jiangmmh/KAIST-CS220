//! Small problems.

use std::iter;

const FAHRENHEIT_OFFSET: f64 = 32.0;
const FAHRENHEIT_SCALE: f64 = 5.0 / 9.0;

/// Converts Fahrenheit to Celsius temperature degree.
pub fn fahrenheit_to_celsius(degree: f64) -> f64 {
    // 转换公式为：C = (F−32) * 5 / 9
    (degree - FAHRENHEIT_OFFSET) * FAHRENHEIT_SCALE
}

/// Capitalizes English alphabets (leaving the other characters intact).
pub fn capitalize(input: String) -> String {
    input
        .chars()
        .map(|c| c.to_ascii_uppercase())
        .collect::<String>()
}

/// Returns the sum of the given array. (We assume the absence of integer overflow.)
pub fn sum_array(input: &[u64]) -> u64 {
    let mut sum = 0;
    for x in input {
        sum += x;
    }
    sum
}

/// Given a non-negative integer, say `n`, return the smallest integer of the form `3^m` that's
/// greater than or equal to `n`.
///
/// For instance, up3(6) = 9, up3(9) = 9, up3(10) = 27. (We assume the absence of integer overflow.)
pub fn up3(n: u64) -> u64 {
    let mut power_of_three: u64 = 1;

    while power_of_three < n {
        power_of_three *= 3;
    }
    power_of_three
}

/// Returns the greatest common divisor (GCD) of two non-negative integers. (We assume the absence
/// of integer overflow.)
pub fn gcd(lhs: u64, rhs: u64) -> u64 {
    let mut tlhs = lhs;
    let mut trhs = rhs;

    if tlhs < trhs {
        std::mem::swap(&mut tlhs, &mut trhs);
    }

    while trhs != 0 {
        let temp = tlhs % trhs;
        tlhs = trhs;
        trhs = temp;
    }

    tlhs
}

/// Returns the array of nC0, nC1, nC2, ..., nCn, where nCk = n! / (k! * (n-k)!). (We assume the
/// absence of integer overflow.)
///
/// Consult <https://en.wikipedia.org/wiki/Pascal%27s_triangle> for computation of binomial
/// coefficients without integer overflow.
pub fn chooses(n: u64) -> Vec<u64> {
    // C(0, 0) = 1
    if n == 0 {
        return vec![1];
    }

    // 根据杨辉三角的计算公式：C(n,k)=C(n,k−1) * (n−k+1) / k
    let mut ans: Vec<u64> = Vec::with_capacity(n as usize + 1);

    // 初始化C(n, 0) = 1
    let mut current_choose: u64 = 1;
    ans.push(current_choose);

    for k in 1..=n {
        // 虽然题目保证了最后的结果不会超过u64的范围，但是numer不保证，因此使用u128避免溢出
        let numerator: u128 = current_choose as u128 * (n - k + 1) as u128;
        let denominator: u128 = k as u128;
        current_choose = (numerator / denominator) as u64;
        ans.push(current_choose);
    }
    ans
}

/// Returns the "zip" of two vectors.
///
/// For instance, `zip(vec![1, 2, 3], vec![4, 5])` equals to `vec![(1, 4), (2, 5)]`. Here, `3` is
/// ignored because it doesn't have a partner.
pub fn zip(lhs: Vec<u64>, rhs: Vec<u64>) -> Vec<(u64, u64)> {
    lhs.into_iter().zip(rhs).collect()
}
