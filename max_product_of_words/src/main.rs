pub fn max_product(words: Vec<&str>) -> i32 {
    let no_common_letters = |s1: &str, s2: &str| -> bool {
        let mut bitmask1 = 0;
        let mut bitmask2 = 0;
        for c in s1.chars() {
            bitmask1 |= 1 << (c as i32 - 'a' as i32);
        }
        for c in s2.chars() {
            bitmask2 |= 1 << (c as i32 - 'a' as i32);
        }

        (bitmask1 & bitmask2) == 0
    };

    let mut largest = 0;

    for (ie, i) in words.iter().enumerate() {
        for (je, j) in words.iter().enumerate() {
            if ie != je && no_common_letters(i, j) {
                largest = std::cmp::max(largest, i.len() * j.len());
            }
        }
    }

    largest as i32
}

fn main() {
    let words1 = vec!["abcw", "baz", "foo", "bar", "xtfn", "abcdef"];
    let words2 = vec!["a", "ab", "abc", "d", "cd", "bcd", "abcd"];
    let words3 = vec!["a", "aa", "aaa", "aaaa"];

    let out1 = max_product(words1);
    let out2 = max_product(words2);
    let out3 = max_product(words3);

    println!("{}", out1);
    println!("{}", out2);
    println!("{}", out3);
}
