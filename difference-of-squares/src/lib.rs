/*
 * If you ever need to solve a problem like this without the internet...
 * 1. Assume the sum of any polynomial is a polynomial of one degree greater.
 * 2. Plug in the first few terms to get a linear equation of the coefficents.
 * 3. Solve.
 * 4. Prove with induction.
 *
 * sum(a*x^2 + b*x + c) = A*x^3 + B*x^2 + C*x + D
 * plug in x = 1, x = 2, x = 3, x = 4, ... until you have enough data to solve.
 */

pub fn square_of_sum(n: u32) -> u32 {
    let acc = n * (n + 1) / 2;
    acc * acc
}

pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    n * (n + 1) * (3 * n + 2) * (n - 1) / 12
}
