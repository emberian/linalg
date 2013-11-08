use std::vec;

/// A two-dimensional matrix.
#[deriving(Clone)]
pub struct Mat2<T> {
    priv data: ~[~[T]],
    priv n: uint,
    priv m: uint,
}

pub struct RowIterator<'self, T> {
    priv mat: &'self Mat2<T>,
    priv i: uint,
}

impl<'self, T> Iterator<&'self [T]> for RowIterator<'self, T> {
    fn next(&mut self) -> Option<&'self [T]> {
        let r = self.mat.get_row_opt(self.i);
        self.i += 1;
        r
    }
}

// TODO: remove clone bound?
impl<T: Default+Clone> Mat2<T> {
    /// Create a new (n x m) matrix, using the Default implementation of T
    pub fn new(n: uint, m: uint) -> Mat2<T> {
        let data = vec::from_elem(n, vec::from_elem(m, Default::default()));

        Mat2 { data: data, n: n, m: m }
    }
}

impl<T> Mat2<T> {
    /// Create a new (n x m) matrix, using `f` to create each element. `f` is given the coordinate
    /// (row, column) for each element it's constructing.
    pub fn new_with(n: uint, m: uint, f: &fn(uint, uint) -> T) -> Mat2<T> {
        let data = vec::from_fn(n, |n| vec::from_fn(m, |m| f(n,m)));

        Mat2 { data: data, n: n, m: m }
    }

    /// Create a new matrix from a vector. Returns None if the inner vectors don't all have the same
    /// length, or if the vector is empty.
    pub fn from_vec(m: ~[~[T]]) -> Option<Mat2<T>> {
        let n = m.len();

        if n == 0 {
            return None;
        }

        let l = m[0].len();

        if m.iter().all(|x| x.len() == l) {
            Some(Mat2 { data: m, n: n, m: l })
        } else {
            None
        }
    }

    /// Return the dimensions of the matrix, (m, n)
    pub fn get_dimension(&self) -> (uint, uint) {
        (self.m, self.n)
    }

    /// Swap two rows. Fails if either of the indices are out of bounds.
    pub fn swap_rows(&mut self, i: uint, j: uint) {
        self.data.swap(i, j);
    }

    /// Set a row to the given vector. Fails if `i` is out of bounds.
    pub fn set_row(&mut self, i: uint, r: ~[T]) {
        self.data[i] = r;
    }

    /// Get the row at `i` as a slice. Fails if `i` is out of bounds.
    pub fn get_row<'a>(&'a self, i: uint) -> &'a [T] {
        self.data[i].as_slice()
    }

    /// Get the row at `i` as a slice. Returns `None` if `i` is out of bounds.
    pub fn get_row_opt<'a>(&'a self, i: uint) -> Option<&'a [T]> {
        self.data.get_opt(i).map(|o| o.as_slice())
    }

    /// Add a column to the matrix.
    pub fn add_column(&mut self, column: ~[T]) {
        // this makes sure the unsafe_mut_ref below will be valid
        assert_eq!(self.n, column.len());

        for (idx, itm) in column.move_iter().enumerate() {
            unsafe { (*self.data.unsafe_mut_ref(idx)).push(itm); }
        }
    }

    /// Iterate over the rows of a matrix.
    pub fn row_iter<'a>(&'a self) -> RowIterator<'a, T> {
        RowIterator {
            mat: self,
            i: 0
        }
    }
}

impl<T: Mul<T, T>> Mat2<T> {
    /// Scale a row by a scalar.
    pub fn scale_row(&mut self, i: uint, a: T) {
        for idx in range(0, self.data[i].len()) {
            self.data[i][idx] = self.data[i][idx] * a;
        }
    }
}

impl<T: Eq> Eq for Mat2<T> {
    fn eq(&self, other: &Mat2<T>) -> bool {
        self.data == other.data
    }
}

impl<T: Mul<T, T> + Add<T, T> + Clone> Mat2<T> {
    /// Add a row `i` scaled by `a` to another row `j`. Fails if either of the indices are out of
    /// bounds.
    fn add_scaled(&mut self, i: uint, j: uint, a: T) {
        let r = self.data[i].iter().enumerate().map(|(i, x)| x.clone() * a + self.data[j][i])
                    .to_owned_vec();
        self.set_row(j, r);
    }
}

#[cfg(test)]
mod tests {
    use super::Mat2;

    #[test]
    fn test_cons() {
        let x: Mat2<int> = Mat2::new(3, 2);

        let y: Option<Mat2<int>> = Mat2::from_vec(~[]);
        assert_eq!(y, None);

        let z: Mat2<int> = Mat2::new_with(3, 2, |_,_| 0);
        assert_eq!(x, z);
    }

    #[test]
    fn test_get_dimension() {
        let x: Mat2<int> = Mat2::from_vec(~[~[1, 2], ~[3, 4]]).unwrap();
        assert_eq!(x.get_dimension(), (2, 2));
    }

    #[test]
    fn test_swap_rows() {
        let mut x = Mat2::from_vec(
            ~[
                ~[1i, 2, 3],
                ~[4, 5, 6],
                ~[7, 8, 9]
            ]).unwrap();
        x.swap_rows(0, 1);
        assert!(x.get_row(0) == &[4, 5, 6]);
        assert!(x.get_row(1) == &[1, 2, 3]);
        assert!(x.get_row(2) == &[7, 8, 9]);
    }

    #[test]
    fn test_get_row() {
        let x = Mat2::from_vec(
            ~[
                ~[1i, 2, 3],
                ~[4, 5, 6],
                ~[7, 8, 9]
            ]).unwrap();
        assert!(x.get_row(0) == &[1, 2, 3]);
        assert!(x.get_row_opt(3) == None);
    }

    #[test]
    fn test_add_column() {
        let mut x = Mat2::from_vec(
            ~[
                ~[1i, 2, 3],
                ~[4, 5, 6],
                ~[7, 8, 9]
            ]).unwrap();
        x.add_column(~[0, 0, 0]);
        assert!(x.get_row(0) == &[1, 2, 3, 0]);
    }

    #[test]
    fn test_row_iter() {
        let x = Mat2::from_vec(
            ~[
                ~[1i, 2, 3],
                ~[4, 5, 6],
                ~[7, 8, 9]
            ]).unwrap();
        let mut it = x.row_iter();
        assert_eq!(it.next().unwrap(), &[1,2,3]);
        assert_eq!(it.next().unwrap(), &[4,5,6]);
        assert_eq!(it.next().unwrap(), &[7,8,9]);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_scale_row() {
        let mut x = Mat2::from_vec(~[~[1i, 1, 1]]).unwrap();
        x.scale_row(0, 3);
        assert!(x.get_row(0) == &[3, 3, 3]);
    }

    #[test]
    fn test_add_scaled() {
        let mut x = Mat2::from_vec(
            ~[
                ~[1i, 2, 3],
                ~[4, 5, 6],
                ~[7, 8, 9]
            ]).unwrap();
        x.add_scaled(0, 1, 1);
        assert!(x.get_row(0) == &[1, 2, 3]);
        assert!(x.get_row(1) == &[5, 7, 9]);
    }
}
