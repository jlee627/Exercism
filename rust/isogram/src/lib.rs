pub fn check(candidate: &str) -> bool {

    // Check if empty string
    if candidate.len() == 0 {
        true
    }

    else {
        // Make string all lowercase and filter out dashes
        let mut isogram = candidate.to_lowercase()
            .chars()
            .filter(|x| !"- ".contains(*x))
            .collect::<Vec<char>>();
        // Sort the string
        isogram.sort();

        // Get rid of any duplicate letters
        let mut clear = isogram.clone();
        clear.dedup();

        // Check to see if clear is the same as isogram after reduction
        isogram == clear
    }
}
