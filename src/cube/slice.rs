/// For a 3x3 cube, slice_index ranges is one of: [-1, 0, 1]
/// For a 4x4 cube, slice_index ranges is one of: [-2, -1, 1, 2]
///
/// column_index ranges from 0..cube_size
pub fn slice_to_column_index(slice: i32, cube_size: usize) -> usize {
    let half_cube_size = cube_size as i32 / 2;

    if cube_size % 2 == 0 {
        if slice.is_positive() {
            return (slice - 1 + half_cube_size) as usize;
        } else {
            return (slice + half_cube_size) as usize;
        }
    } else {
        return (slice + (cube_size - 1) as i32 / 2) as usize;
    }
}

/// column_index ranges from 0..cube_size
///
/// For a 3x3 cube, slice_index ranges is one of: [-1, 0, 1]
/// For a 4x4 cube, slice_index ranges is one of: [-2, -1, 1, 2]
pub fn column_index_to_slice(column_index: i32, cube_size: usize) -> i32 {
    let half_cube_size = cube_size as i32 / 2;

    if cube_size % 2 == 0 {
        if column_index + 1 <= half_cube_size {
            return (half_cube_size - column_index) * -1;
        } else {
            return column_index - half_cube_size + 1;
        }
    } else {
        return column_index - half_cube_size;
    }
}

#[cfg(test)]
mod test {
    use crate::cube::slice::{column_index_to_slice, slice_to_column_index};

    #[test]
    fn test_slice_to_column_index() {
        // 3x3
        assert_eq!(0, slice_to_column_index(-1, 3));
        assert_eq!(1, slice_to_column_index(0, 3));
        assert_eq!(2, slice_to_column_index(1, 3));

        // 4x4
        assert_eq!(0, slice_to_column_index(-2, 4));
        assert_eq!(1, slice_to_column_index(-1, 4));
        assert_eq!(2, slice_to_column_index(1, 4));
        assert_eq!(3, slice_to_column_index(2, 4));
    }

    #[test]
    fn test_column_index_to_slice() {
        // 3x3
        assert_eq!(-1, column_index_to_slice(0, 3));
        assert_eq!(0, column_index_to_slice(1, 3));
        assert_eq!(1, column_index_to_slice(2, 3));

        // 4x4
        assert_eq!(-2, column_index_to_slice(0, 4));
        assert_eq!(-1, column_index_to_slice(1, 4));
        assert_eq!(1, column_index_to_slice(2, 4));
        assert_eq!(2, column_index_to_slice(3, 4));
    }
}
