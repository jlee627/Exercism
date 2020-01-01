pub fn series(digits: &str, len: usize) -> Vec<String> {
    // Create new vector
    let mut s = Vec::new();

    // If desired vector length is too much just return number
    if len > digits.len() {
        return s;
    }

    // Add strings to vector of the desired length
    for i in 0..(digits.len() - len + 1) {
        s.push(digits[i..(i+len)].to_string());
    }
    
    // Return series
    s
}
