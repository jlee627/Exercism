pub fn find() -> Option<u32> {

    // Iterate through every number we know it can be (1-1000)
    for a in 1..1000 {
        for b in 1..a {
            // Find C for a + b + c = 1000
            let c = 1000 - a - b;

            // Check to see if it still fits in pythagorean theorem
            if (c * c == b * b + a * a) == true {
                // Return product of pythagoren triplet
                return Some(a * b * c);
            }
        }
    }

    // No triplet found
    None
}
