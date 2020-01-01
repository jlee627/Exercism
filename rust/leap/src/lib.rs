pub fn is_leap_year(year: i32) -> bool {

    // If year is divisible by 4 and not 100 it is a leap year
    if year % 4 == 0 && year % 100 != 0 {
        return true;
    }
    // If year is divisible by 400 it is a leap year
    if year % 400 == 0 {
        return true;
    }

    // Not a leap year
    false
}
