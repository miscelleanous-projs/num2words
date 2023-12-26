pub fn number_to_words(n: i64) -> String {
    const MULTIPLIERS: [&str; 5] = ["", "Thousand", "Million", "Billion", "Trillion"];
    
    const FIRST_TWENTY: [&str; 20] = [
        "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
        "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen",
    ];

    const TENS: [&str; 10] = ["", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];

    fn convert_hundreds(curr_hun: i64, first_twenty: &[&str; 20]) -> String {
        let mut result = String::new();

        if curr_hun >= 100 {
            result += &format!("{} Hundred", first_twenty[(curr_hun / 100) as usize]);
        }
        
        let curr_ten = curr_hun % 100;

        if curr_ten > 0 {
            if !result.is_empty() {
                result += " ";
            }

            if curr_ten < 20 {
                result += first_twenty[curr_ten as usize];
            } else {
                result += &format!("{}", TENS[(curr_ten / 10) as usize]);

                if curr_ten % 10 != 0 {
                    result += &format!(" {}", first_twenty[(curr_ten % 10) as usize]);
                }
            }
        }

        result
    }

    let mut answer = String::new();
    let mut multiplier_index = 0;
    let mut num = n;

    if num == 0 {
        return "Zero".to_string();
    }

    while num > 0 {
        let curr_hun = num % 1000;

        if curr_hun > 0 {
            let current_part = convert_hundreds(curr_hun, &FIRST_TWENTY);
            if !current_part.is_empty() {
                if !answer.is_empty() {
                    answer = format!("{} {} {}", current_part, MULTIPLIERS[multiplier_index], answer);
                } else {
                    answer = format!("{}{}", current_part, answer);
                }
            }
        }

        num /= 1000;
        multiplier_index += 1;
    }

    answer.trim().to_string()
}
