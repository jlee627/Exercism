pub fn build_proverb(list: Vec<&str>) -> String {
    let mut proverb = String::new();

    // If no input return empty string
    if list.len() == 0 {
        return proverb;
    }

    // Otherwise make proverb
    if list.len() > 1 {
        proverb += &format!("For want of a {} the {} was lost.\n", list[0], list[1]);

        // Iterate through input while adding tothe proverb
        if list.len() > 2 {
            for x in 1..list.len()-1 {
                proverb += &format!("For want of a {} the {} was lost.\n", list[x], list[x+1])
            }
        }
    }

    // Add final (or only if only 1 input) line of proverb
    proverb += &format!("And all for the want of a {}.", list[0]);

    // Return proverb
    proverb
}
