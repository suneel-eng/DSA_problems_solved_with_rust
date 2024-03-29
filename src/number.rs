// ============== Problem ==================
// Given an integer [number], you need to find the count of it's factors.
// Factor of a number is the number which divides it perfectly leaving no remainder.
// Example : 1, 2, 3, 6 are factors of 6
// ==========================================
pub fn count_factors(number: u64) -> u64 {

    let mut current_factor: u64 = 1;
    let mut total_factors: u64 = 0;

    while current_factor <= number {
        if number % current_factor == 0 {
            total_factors += 1;
        }
        current_factor += 1;
    }

    total_factors
}

pub fn is_prime_number(number: u64) -> u8 {
    // Write your code here.
    let total_factors = count_factors(number);

    if total_factors == 2 {
        return 1;
    }

    0
}

// ======== Problem ===========
// Given a number [number]. Return square root of the number if it is perfect square otherwise return -1.
// Note: A number is a perfect square if its square root is an integer.
// ===========================
pub fn square_root(number: u64) -> i64 {
    // write your code here
    let mut current_factor: u64 = 1;

    while current_factor <= number {

        if (number/current_factor == current_factor) && (number%current_factor == 0) {
            return current_factor as i64;
        }
        current_factor += 1;
    }

    -1
}

// ================ Problem =============
// You are given an integer [number]. You have to tell whether it is a perfect number or not.
// Perfect number is a positive integer which is equal to the sum of its proper positive divisors.
// A proper divisor of a natural number is the divisor that is strictly less than the number.
// =========================================
pub fn is_perfect_number(number: u64) -> bool {
    // Write your code here
    let mut total_of_divisors: u64 = 0;

    for i in 1..(number) {
        if number%i == 0 {
            total_of_divisors += i;
        }
    }

    if total_of_divisors == number {
        return true;
    }

    false
}

// Problem: You will be given an integer [number]. You need to return the count of prime numbers less than or equal to [number].
pub fn count_of_prime_numbers(number: u64) -> u64 {
    // Write your code here.
   let mut prime_numbers_count: u64 = 0;

  for i in 1..=(number) {
   let factors_count = count_factors(i);

   if factors_count == 2 {
       prime_numbers_count += 1;
   }

  }


  prime_numbers_count

}