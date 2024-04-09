// ========= Problem ============
// Given an array [a] of N integers. 
// Count the number of elements that have at least 1 elements greater than itself.
// ===============================
pub fn count_elements(array: Vec<u64>) -> u64 {
    // Write your code here
    let mut current_max_element = array[0];
    let mut total_max_elements = 1;

    for index in 1..(array.len()) {
        
        let element = array[index];

        if element == current_max_element {
            total_max_elements += 1;
        }

        if element > current_max_element {
            current_max_element = element;
            total_max_elements = 1;
        }
    }

    (array.len() - total_max_elements) as u64
}

// =========== Problem =============
// Given an array [a] and an integer [b].
// A pair(i, j) in the array is a good pair if i != j and (a[i] + a[j] == b).
// Check if any good pair exist or not.
// ==================================
pub fn is_good_pair_exist(a: Vec<u64>, b: u64) -> bool {

    use std::collections::HashMap;

    let mut map: HashMap<u64, usize> = HashMap::new();

    for (index, element) in a.iter().enumerate() {
        
        let remaining = if *element > b {
            continue;
        } else {
            b - element
        };

        if map.contains_key(&remaining) {
            if let Some(value) = map.get(&remaining) {
                if value != &index {
                    return true;
                }
            }
        } else {
            map.insert(*element, index);
        }
    }

    false
}

// ============ Problem ==============
// Given an array [a] of N integers and also given two integers [b] and [c].
// Reverse the elements of the array [a] within the given inclusive range [b, c].
// ===================================
pub fn reverse_in_range(a: Vec<u64>, b: usize, c: usize) -> Vec<u64> {

    let (mut max, mut min) = match b > c {
        true => (b, c),
        false => (c, b)
    };

    let mut a = a.clone();

    while min < max {

        let temp = a[min];
        a[min] = a[max];
        a[max] = temp;

        min += 1;
        max -= 1;
    }

    a
}

pub fn possible_sub_arrays(array: Vec<u8>) -> Vec<Vec<u8>> {
    
    let mut sub_arrays: Vec<Vec<u8>> = Vec::new();

    for i in 0..array.len() {

        let mut sub_array: Vec<u8> = Vec::new();

       for j in i..array.len() {

            sub_array.push(array[j]);

            sub_arrays.push(sub_array.clone());
       }
    }

    sub_arrays
}