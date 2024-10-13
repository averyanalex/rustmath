use std::{
    fmt::{self, Debug, Display},
    io::stdin,
    ops::Mul,
};

use num::{BigRational, FromPrimitive, Zero};
use rayon::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix<T>(Vec<Vec<T>>);

impl<const I: usize, const J: usize> From<[[i32; J]; I]> for Matrix<BigRational> {
    fn from(value: [[i32; J]; I]) -> Self {
        Self(
            value
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|n| BigRational::from_i32(n).unwrap())
                        .collect()
                })
                .collect(),
        )
    }
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for (idx, elem) in row.iter().enumerate() {
                write!(f, "{elem}")?;
                if idx != row.len() - 1 {
                    write!(f, "\t")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Mul for Matrix<BigRational> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.0.len() > 1);
        assert_eq!(self.0[0].len(), rhs.0.len());
        let m = rhs.0.len();

        let mat = (0..self.0.len())
            .map(|i| {
                (0..rhs.0[0].len())
                    .map(|j| (0..m).map(|idx| &self.0[i][idx] * &rhs.0[idx][j]).sum())
                    .collect()
            })
            .collect();
        Self(mat)
    }
}

fn main() {
    let mut mat = Vec::new();
    for line in stdin().lines() {
        mat.push(
            line.unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        );
    }

    let mut mat = Matrix(mat);

    gauss(&mut mat);

    println!("===========================================================");
    print!("{mat}");
    println!("===========================================================");

    // let a = Matrix::from([[1, 0, 0], [0, 1, 0], [0, 0, 1], [8, -1, -6]]);
    // let b = Matrix::from([[1, 0, 2, 2], [0, 1, 6, 5], [3, 3, 24, 21]]);
    // let c1 = Matrix::from([
    //     [-11, 6, 1, -1],
    //     [-10, 5, 1, -1],
    //     [-2, 2, 1, 0],
    //     [10, -6, -2, 1],
    // ]);
    // print!("{}", a * b * c1);
}

fn gauss(mat: &mut Matrix<BigRational>) {
    let columns = mat.0[0].len();
    let rows = mat.0.len();
    for row in mat.0.iter().skip(1) {
        assert_eq!(row.len(), columns);
    }

    let mut cur_row_idx = 0;

    for col in 0..rows {
        if let Some(non_zero_row) =
            mat.0
                .iter()
                .enumerate()
                .skip(cur_row_idx)
                .find_map(|(row_idx, row)| {
                    if !row[col].is_zero() {
                        Some(row_idx)
                    } else {
                        None
                    }
                })
        {
            mat.0.swap(non_zero_row, cur_row_idx);

            let factor = mat.0[cur_row_idx][col].clone();
            mat.0[cur_row_idx].par_iter_mut().for_each(|col| {
                *col /= &factor;
            });

            let cur_row = mat.0[cur_row_idx].clone();
            mat.0
                .par_iter_mut()
                .enumerate()
                .filter(|(row_idx, _)| *row_idx != cur_row_idx)
                .map(|(_, row)| row)
                .for_each(|row| {
                    let factor = row[col].clone();

                    for (col, cur_col) in row.iter_mut().zip(cur_row.iter()) {
                        *col -= cur_col * &factor;
                    }
                });

            cur_row_idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::FromPrimitive;

    #[test]
    fn simple_gauss() {
        let mut mat = Matrix::from([[3, 2, -5, -1], [2, -1, 3, 13], [1, 2, -1, 9]]);
        gauss(&mut mat);
        assert_eq!(
            mat,
            Matrix::from([[1, 0, 0, 3], [0, 1, 0, 5], [0, 0, 1, 4]]),
        );
    }

    #[test]
    fn big_gauss() {
        let mut mat = Matrix(
            (0..100)
                .map(|i| {
                    (0..100)
                        .map(|j| i * 100 + j)
                        .map(|x| BigRational::from_i32(x).unwrap())
                        .collect()
                })
                .collect(),
        );

        gauss(&mut mat);
    }

    #[test]
    fn test_mul() {
        let a = Matrix::from([[1, 0, 0], [0, 1, 0], [0, 0, 1], [8, -1, -6]]);
        let b = Matrix::from([[1, 0, 2, 2], [0, 1, 6, 5], [3, 3, 24, 21]]);
        let c = Matrix::from([
            [1, 0, 2, 2],
            [0, 1, 6, 5],
            [3, 3, 24, 21],
            [-10, -19, -134, -115],
        ]);

        assert_eq!(a * b, c);
    }
}
