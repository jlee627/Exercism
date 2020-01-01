pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut sp = Vec::new();

    // Iterate through input
    for i in 0..input.len() {
        let row = &input[i];

        for j in 0..row.len() {
            let item = row[j];

            // Check to see if item is greater than everything in its rows
            let greater = row.iter().all(|x| item >= *x);
            // Check to see if item is less than everything its column
            let less = input.iter().map(|row| &row[j]).all(|x| item <= *x);

            // If yes then saddle point found
            if greater && less {
                sp.push((i, j));
            }
        }
    }

    sp
}
