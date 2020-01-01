pub fn encode(n: u64) -> String {
    // Format the number
    let mut num = format!("{}", n);

    // If there is only a zero, return "zero"
    if n == 0 {
        return "zero".to_string();
    }

    // If there is no number return empty string
    if num.len() == 0 {
        return "".to_string();
    }

    // Going to split the number into groups of 3 to help with naming convention
    let zeros = num.len() % 3;

    // See if there are any zeros we need to add to make all numbers fit into groups of 3s
    if zeros != 0 {
        let mut i = 0;

        // Add zeros to number
        while i != 3 - zeros {
          num.insert(0, '0');
          i += 1;
            }
        }

    let section = num.chars().collect::<Vec<char>>();
    let mut groups: Vec<&str> = vec![];

    // Iterate through each set of 3 numbers and name accordingly
    for (set, part) in section.chunks(3).enumerate() {
        let hundreds = part[0];
        let tens = part[1];
        let ones = part[2];

        if hundreds != '0' {
            let h = match hundreds {
                '1' => "one hundred",
                '2' => "two hundred",
                '3' => "three hundred",
                '4' => "four hundred",
                '5' => "five hundred",
                '6' => "six hundred",
                '7' => "seven hundred",
                '8' => "eight hundred",
                '9' => "nine hundred",
                _   => "",
                };

                // Add which hundred we're at to name
                groups.push(h);
            }

        if tens == '1' {
            let teens = match ones {
                '0' => "ten",
                '1' => "eleven",
                '2' => "twelve",
                '3' => "thirteen",
                '4' => "fourteen",
                '5' => "fifteen",
                '6' => "sixteen",
                '7' => "seventeen",
                '8' => "eighteen",
                '9' => "nineteen",
                _   => "",
            };
            // Add which teen we're at to name
            groups.push(teens);
        }

        else if tens != '0' {
            let t = match tens {
                '1' => "ten",
                '2' => "twenty",
                '3' => "thirty",
                '4' => "forty",
                '5' => "fifty",
                '6' => "sixty",
                '7' => "seventy",
                '8' => "eighty",
                '9' => "ninety",
                _   => "",
            };
            // Add which ten we're at to name
            groups.push(t);
        }

        if ones != '0' && tens != '1' {
            let o = match ones {
                '1' => "one",
                '2' => "two",
                '3' => "three",
                '4' => "four",
                '5' => "five",
                '6' => "six",
                '7' => "seven",
                '8' => "eight",
                '9' => "nine",
                _   => "",
            };

            // Add a hyphen if tens is 20,30,etc
            if tens != '0' && tens != '1' {
                groups.push("-");
            }
            // Add which ones we're at to name
            groups.push(o);
        }

        // If current chunk is not purely zero then find magnitude
        if hundreds != '0' || tens != '0' || ones != '0' {
            // Calcualte based on length minus set we're at
            let exp = (num.len() / 3) - set;

            // Find magnitude based on size
            let size = match exp {
                1 => "",
                2 => "thousand",
                3 => "million",
                4 => "billion",
                5 => "trillion",
                6 => "quadrillion",
                7 => "quintillion",
                _ => "",
            };
            // Add which hundred we're at to name
            groups.push(size);
        }
    }

    // Combine all strings together
    groups.join(" ").trim().replace(" - ", "-").to_string()
}
