//! Doing operations on a matrix as if it were a system of linear equations

use std::num::{Zero, zero};
use matrix::Mat2;

pub fn substitute<T: Clone + Zero + Mul<T, T> + Add<T, T>>
       (matrix: &Mat2<T>, values: &[T]) -> Mat2<T> {

    let (_, n) = matrix.get_dimension();
    assert_eq!(n, values.len());
    Mat2::new_with(n, 1, |_,n| matrix.get_row(n).iter()
                               .enumerate()
                               .fold(zero::<T>(), |a, (i, b)| a + values[i]*(*b).clone()))
}

#[cfg(test)]
mod test {
    use super::*;
    use matrix::Mat2;

    #[test]
    fn test_substitute() {
        let m = Mat2::new_with(2, 2, |_,_| 1i);
        let r = substitute(&m, &[2i, 3i]);
        let m2 = Mat2::from_vec(~[~[5], ~[5]]).unwrap();
        assert_eq!(r, m2);
    }
}
