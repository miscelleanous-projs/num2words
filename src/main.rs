mod num2words;

use num2words::number_to_words;

fn main() {
    let cases = vec![7, 36, 666, 9_564, 123_456_789, 999_999_999, 10_101_010_110_001];

    println!("\nNumber to Words:\n");

    for &n in &cases {
        println!("{} => {}", n, number_to_words(n));
    }

    println!("");
}
