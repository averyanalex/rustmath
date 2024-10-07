use std::{
    fmt::{self, Debug, Display},
    io::BufRead,
    ops::{Div, Mul, Sub},
    str::FromStr,
};

use num::{BigRational, Zero};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Matrix<T>(Vec<Vec<T>>);

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

fn read_nums<T>(io: &mut impl BufRead) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut buf = String::new();
    io.read_line(&mut buf).unwrap();

    buf.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn main() {
    let mut stdin = std::io::stdin().lock();

    let mut mat = vec![read_nums::<BigRational>(&mut stdin)];
    for _ in 1..mat[0].len() - 1 {
        mat.push(read_nums(&mut stdin));
    }

    let mut mat = Matrix(mat);

    gauss(&mut mat);

    println!("===========================================================");
    print!("{mat}");
    println!("===========================================================");

    if mat.0.iter().any(|row| {
        !row.last().unwrap().is_zero() && row[..row.len() - 1].iter().all(|elem| elem.is_zero())
    }) {
        println!("No solution");
    } else {
        for (row_idx, row) in mat.0.iter().enumerate() {
            if let Some(non_zero_idx) = row.iter().position(|elem| !elem.is_zero()) {
                let mut answer = format!("x_{} = {}", row_idx + 1, row.last().unwrap());
                for (idx, elem) in row.iter().enumerate().skip(non_zero_idx + 1) {
                    if idx != row.len() - 1 && !elem.is_zero() {
                        answer += &format!(" - {} * x_{}", elem, idx + 1);
                    }
                }
                answer += &format!(" , i.e. {}", row.last().unwrap());
                println!("{}", answer);
            } else {
                println!("x_{} is free, i.e. 0", row_idx + 1);
            }
        }
    }
}

fn gauss<T>(mat: &mut Matrix<T>)
where
    T: Clone,
    T: PartialEq<T>,
    T: Zero,
    T: Div<Output = T>,
    T: Mul<Output = T>,
    T: Sub<Output = T>,
    T: Debug,
{
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
                    if row[col] != T::zero() {
                        Some(row_idx)
                    } else {
                        None
                    }
                })
        {
            mat.0.swap(non_zero_row, cur_row_idx);

            let factor = mat.0[cur_row_idx][col].clone();
            for col in mat.0[cur_row_idx].iter_mut() {
                *col = col.clone() / factor.clone();
            }

            for row_idx in 0..rows {
                if row_idx != cur_row_idx {
                    let factor = mat.0[row_idx][col].clone();

                    for col_idx in 0..columns {
                        mat.0[row_idx][col_idx] = mat.0[row_idx][col_idx].clone()
                            - mat.0[cur_row_idx][col_idx].clone() * factor.clone();
                    }
                }
            }

            cur_row_idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use num::FromPrimitive;

    use super::*;

    fn to_rational_mat(mat: Vec<Vec<i64>>) -> Matrix<BigRational> {
        Matrix(
            mat.into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|n| BigRational::from_i64(n).unwrap())
                        .collect()
                })
                .collect(),
        )
    }

    #[test]
    fn simple_gauss() {
        let mut mat = to_rational_mat(vec![
            vec![3, 2, -5, -1],
            vec![2, -1, 3, 13],
            vec![1, 2, -1, 9],
        ]);
        gauss(&mut mat);
        assert_eq!(
            mat,
            to_rational_mat(vec![vec![1, 0, 0, 3], vec![0, 1, 0, 5], vec![0, 0, 1, 4]]),
        );
    }
}
