#[cfg(test)]

mod tests {
    use crate::matrix_2d::sum_of_given_range;

    use super::print_all_elements_of_2d_matrix;

    #[test]
    fn test_print_all_elements_of_2d_matrix() {
        
        let matrix: Vec<Vec<u8>> = vec![
            vec![ 1, 2 ],
            vec![ 3, 4 ]
        ];

        let result = print_all_elements_of_2d_matrix(matrix);

        assert_eq!(result, vec![ 1, 2, 3, 4 ]);
    }

    #[test]
    fn test_sum_of_given_range() {
        
        let matrix: Vec<Vec<u8>> = vec![
            vec![ 1, 2, 3, 4 ],
            vec![ 5, 6, 7, 8 ],
            vec![ 9, 10, 11, 12 ],
            vec![ 13, 14, 15, 16 ]
        ];

        let result = sum_of_given_range(matrix, (0, 1), (1, 2 ));

        assert_eq!(result, 18);
    }
}

fn print_all_elements_of_2d_matrix(matrix: Vec<Vec<u8>>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for row in matrix {
        for value in row {
            result.push(value)
        }
    }

    result
}

fn sum_of_given_range(matrix: Vec<Vec<u8>>, s: (usize, usize), e: (usize, usize)) -> u64 {

    let mut sum: u64 = 0;

    for row_index in s.0..=e.0 {
        for column_index in s.1..=e.1 {
            sum += matrix[row_index][column_index] as u64;
        }
    }

    sum
}