pub fn collatz(n: u64) -> Option<u64> {
    let mut counter = 0;
    let mut num = n;

    // If no number return nothing
    if n == 0 {
        return None;
    }

    // Perform until we get to 1
    while num != 1 {
        // If number is even. divide by 2
        if num % 2 == 0 {
            num = num / 2;
        }
        // If number is odd, times by 3 and add 1
        else {
            num = num * 3 + 1;
        }

        // Increment Counter
        counter += 1;
    }

    // Return counter size
    Some(counter)
}
