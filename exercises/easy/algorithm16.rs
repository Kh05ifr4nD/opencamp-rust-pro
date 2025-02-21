/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place.
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

pub fn rotate_matrix_90_degrees(mat: &mut Vec<Vec<i32>>) {
  if mat.is_empty() || mat[0].is_empty() {
    return;
  }

  let n = mat.len();
  let m = mat[0].len();

  let size = n.max(m);
  for row in mat.iter_mut() {
    row.resize(size, 0);
  }
  while mat.len() < size {
    mat.push(vec![0; size]);
  }

  for lyr in 0..size / 2 {
    let first = lyr;
    let last = size - 1 - lyr;
    for i in first..last {
      let ofst = i - first;
      let top = mat[first][i];
      mat[first][i] = mat[last - ofst][first];
      mat[last - ofst][first] = mat[last][last - ofst];
      mat[last][last - ofst] = mat[i][last];
      mat[i][last] = top;
    }
  }

  if n != m {
    let n_new = m;
    let m_new = n;
    mat.truncate(n_new);
    for row in mat.iter_mut() {
      row.truncate(m_new);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rotate_matrix_1() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    rotate_matrix_90_degrees(&mut matrix);
    println!("Rotated matrix: {matrix:?}");
    assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3],]);
  }

  #[test]
  fn test_rotate_matrix_2() {
    let mut matrix = vec![vec![1, 2], vec![3, 4]];
    rotate_matrix_90_degrees(&mut matrix);
    println!("Rotated matrix: {matrix:?}");
    assert_eq!(matrix, vec![vec![3, 1], vec![4, 2],]);
  }

  #[test]
  fn test_rotate_matrix_3() {
    let mut matrix = vec![vec![1]];
    rotate_matrix_90_degrees(&mut matrix);
    println!("Rotated matrix: {matrix:?}");
    assert_eq!(matrix, vec![vec![1],]);
  }

  #[test]
  fn test_rotate_matrix_4() {
    let mut matrix = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    rotate_matrix_90_degrees(&mut matrix);
    println!("Rotated matrix: {matrix:?}");
    assert_eq!(matrix, vec![vec![5, 3, 1], vec![6, 4, 2],]);
  }
}
