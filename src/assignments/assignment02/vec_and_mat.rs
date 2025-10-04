//! Vector and matrices.
//!
//! You will implement simple operations on vectors and matrices.

use std::cmp::PartialEq;
use std::ops::Mul;

/// 2x2 matrix of the following configuration:
///
/// a, b
/// c, d
#[derive(Debug, Clone, Copy)]
struct Mat2 {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

/// 2x1 matrix of the following configuration:
///
/// a
/// b
#[derive(Debug, Clone, Copy)]
struct Vec2 {
    a: u64,
    b: u64,
}

impl Mat2 {
    /// Creates an identity matrix.
    fn new() -> Self {
        Self {
            a: 1,
            b: 0,
            c: 0,
            d: 1,
        }
    }
}

impl Mul<Mat2> for Mat2 {
    type Output = Mat2;

    /// Consult <https://www.mathsisfun.com/algebra/matrix-multiplying.html>
    fn mul(self, rhs: Mat2) -> Self::Output {
        // 假设 Mat2 的字段 a, b, c, d 都是 u64 类型

        // 关键步骤：使用 u128 进行中间计算
        let new_a = (self.a as u128 * rhs.a as u128) + (self.b as u128 * rhs.c as u128);
        let new_b = (self.a as u128 * rhs.b as u128) + (self.b as u128 * rhs.d as u128);
        let new_c = (self.c as u128 * rhs.a as u128) + (self.d as u128 * rhs.c as u128);
        let new_d = (self.c as u128 * rhs.b as u128) + (self.d as u128 * rhs.d as u128);

        // 如果我们确信最终结果不会溢出 u64，则可以直接 as u64 转换
        // 如果最终结果可能溢出 u64，你的函数签名和 Mat2 结构体需要返回或存储更大的类型。
        Mat2 {
            a: new_a as u64,
            b: new_b as u64,
            c: new_c as u64,
            d: new_d as u64,
        }
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Vec2;

    /// Multiplies the matrix by the vector.
    ///
    /// Consult <https://www.mathsisfun.com/algebra/matrix-multiplying.html>
    fn mul(self, rhs: Vec2) -> Self::Output {
        let mut ans = Vec2 { a: 0, b: 0 };
        ans.a = self.a * rhs.a + self.b * rhs.b;
        ans.b = self.c * rhs.a + self.d * rhs.b;
        ans
    }
}

impl Mat2 {
    /// Calculates the power of matrix.
    fn power(self, power: u64) -> Mat2 {
        if power == 0 {
            return Mat2::new();
        }

        // 矩阵快速幂，将矩阵乘法的次数从m次减少到log2(m)次
        let mut base = self;
        let mut result = Mat2::new();
        let mut p = power;

        while p > 0 {
            if p & 1 == 1 {
                result = result.mul(base);
            }

            p >>= 1;
            base = base.mul(base);
        }
        result
    }
}

impl Vec2 {
    /// Gets the upper value of vector.
    fn get_upper(self) -> u64 {
        std::cmp::max(self.a, self.b)
    }
}

/// The matrix used for calculating Fibonacci numbers.
const FIBONACCI_MAT: Mat2 = Mat2 {
    a: 1,
    b: 1,
    c: 1,
    d: 0,
};

/// The vector used for calculating Fibonacci numbers.
const FIBONACCI_VEC: Vec2 = Vec2 { a: 1, b: 0 };

/// Calculates the Fibonacci number. (We assume the absence of integer overflow.)
///
/// Consult <https://web.media.mit.edu/~holbrow/post/calculating-fibonacci-numbers-with-matrices-and-linear-algebra/> for matrix computation of Fibonacci numbers.
pub fn fibonacci(n: u64) -> u64 {
    (FIBONACCI_MAT.power(n) * FIBONACCI_VEC).get_upper()
}

/// 2x2 floating-point matrix of the following configuration:
///
/// a, b
/// c, d
#[derive(Debug, Clone, Copy)]
pub struct FMat2 {
    /// row 1, column 1
    pub a: f64,
    /// row 1, column 2
    pub b: f64,
    /// row 2, column 1
    pub c: f64,
    /// row 2, column 2
    pub d: f64,
}

impl FMat2 {
    /// Returns the inverse of the given matrix. (We assume the given matrix is always invertible.)
    /// HINT: <https://www.mathcentre.ac.uk/resources/uploaded/sigma-matrices7-2009-1.pdf>
    ///
    /// # Example
    ///
    /// ```ignore
    /// assert_eq!(
    ///     FMat2 { a: 1.0, b: 1.0, c: 2.0, d: 3.0 }.inverse(),
    ///     FMat2 { a: 3.0, b: -1.0, c: -2.0, d: 1.0 }
    /// );
    /// ```
    pub fn inverse(self) -> Self {
        let mut ans = FMat2 {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            d: 0.0,
        };
        let det = self.a * self.d - self.b * self.c;
        let rev_det = 1.0 / det;
        ans.a = rev_det * self.d;
        ans.b = rev_det * -self.b;
        ans.c = rev_det * -self.c;
        ans.d = rev_det * self.a;
        ans
    }
}

// Equivalence between two floating-point matrices, as element-wise equivalence
impl PartialEq for FMat2 {
    fn eq(&self, other: &FMat2) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c && self.d == other.d
    }
}
