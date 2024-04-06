mod number;
mod array;
mod linked_list;

#[cfg(test)]
mod tests {
    use crate::number;
    use crate::array;
    use crate::linked_list;

    #[test]
    fn test_count_factors() {
        let result = number::count_factors(5);
        assert_eq!(result, 2);

        let result = number::count_factors(10);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_prime_number() {
        let result = number::is_prime_number(5);
        assert_eq!(result, 1);

        let result = number::is_prime_number(10);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_perfect_square() {
        let result = number::square_root(5);
        assert_eq!(result, -1);

        let result = number::square_root(4);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_perfect_number() {
        let result = number::is_perfect_number(4);
        assert_eq!(result, false);

        let result = number::is_perfect_number(6);
        assert_eq!(result, true);
    }

    #[test]
    fn test_count_prime_numbers() {
        let result = number::count_of_prime_numbers(19);
        assert_eq!(result, 8);

        let result = number::count_of_prime_numbers(1);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_count_elements() {
        let result = array::count_elements(vec![3, 1, 2]);
        assert_eq!(result, 2);
        
        let result = array::count_elements(vec![5, 5, 3]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_good_pair() {
        let result = array::is_good_pair_exist(vec![1,2,3,4], 7);
        assert_eq!(result, true);
        
        let result = array::is_good_pair_exist(vec![1,2,4], 4);
        assert_eq!(result, false);

        let result = array::is_good_pair_exist(vec![1,2,2], 4);
        assert_eq!(result, true);
    }

    #[test]
    fn test_reverse_range() {
        let result = array::reverse_in_range(vec![1, 2, 3, 4], 2, 3);
        assert_eq!(result, vec![1, 2, 4, 3]);
        
        let result = array::reverse_in_range(vec![2, 5, 6], 0, 2);
        assert_eq!(result, vec![6, 5, 2]);
    }

    #[test]
    fn test_create_linked_list() {

        let result = linked_list::vector_to_linked_list(vec![ 1, 2, 3, 4, 5 ]);

        let answer = Option::Some(Box::new(
            linked_list::ListNode {
                data: 1,
                next: Option::Some(Box::new(
                    linked_list::ListNode {
                        data: 2,
                        next: Option::Some(Box::new(
                            linked_list::ListNode {
                                data: 3,
                                next: Option::Some(Box::new(
                                    linked_list::ListNode {
                                        data: 4,
                                        next: Option::Some(Box::new(
                                            linked_list::ListNode {
                                                data: 5,
                                                next: Option::None
                                            }
                                        ))
                                    }
                                ))
                            }
                        ))
                    }
                ))
            }
        ));

        assert_eq!(result, answer);
    }

    #[test]
    fn test_print_linked_list() {
        let linked_list_head = linked_list::vector_to_linked_list(vec![ 1, 2, 3, 4, 5 ]);

        let result = linked_list::print_linked_list(linked_list_head);

        assert_eq!(result, vec![ 1, 2, 3, 4, 5 ]);
    }
}
