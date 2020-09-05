
//Given a string reverse each word
//Returns string with reversed words
fn reverse_words(str: &str) -> String {
    
    let mut return_string = String::new();
    let mut last_index = 0;
    for (i, c) in str.chars().enumerate() {
        if c.is_whitespace() {
            return_string.push_str(&str[last_index..i].chars().rev().collect::<String>());
            return_string.push_str(" ");
            last_index = i + 1;
        }
        if i == str.chars().count() - 1{
            return_string.push_str(&str[last_index..i + 1].chars().rev().collect::<String>());
        }
    }

    return_string
}

fn descending_order(x: u64) -> u64 {
    
    let mut y: Vec<char> = x.to_string().chars().collect();
    y.sort_by(|a, b| b.cmp(a));

    let s: String = y.into_iter().collect();
    let value:u64 = s.parse::<u64>().unwrap();
    value
}

fn reverse_string(phrase: &str) -> String {
    phrase.chars().rev().collect::<String>()
}

//Given a volume, return number of cubes that fits (descending order n^3 n-1^3 etc), if none return -1
fn find_nb(n: u64) -> i32 {
    let mut i:u64 = 1;
    let mut run = true;
    let mut return_cubes : i32 = -1;
    let mut sum = 0;
    
    while run {
        sum += i.pow(3);
        
        if sum < n {
            i += 1;
        }
        else if sum == n {
            run = false;
            return_cubes = i as i32;
        }
        else if sum > n {
            return_cubes = -1;
            run = false;
        }
    }

    return_cubes
}

//Get time in ms
fn past(h: i32, m: i32, s: i32) -> i32 {
    (s * 1000) + (m * 60 * 1000) + (h * 60 * 60 * 1000)
}

fn main() {
    assert_eq!(reverse_words("The quick brown fox jumps over the lazy dog."), "ehT kciuq nworb xof spmuj revo eht yzal .god");
    assert_eq!(reverse_words("apple"), "elppa");
    assert_eq!(reverse_words("a b c d"),"a b c d");
    assert_eq!(reverse_words("double  spaced  words"), "elbuod  decaps  sdrow");

    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);

    assert_eq!(reverse_string("world"), "dlrow");

    assert_eq!(find_nb(4183059834009), 2022);
    assert_eq!(find_nb(24723578342962), -1);
    assert_eq!(find_nb(135440716410000), 4824);
    assert_eq!(find_nb(40539911473216), 3568);
    assert_eq!(find_nb(26825883955641), 3218);

    assert_eq!(past(0, 1, 1), 61000);
    assert_eq!(past(1, 1, 1), 3661000);
    assert_eq!(past(0, 0, 0), 0);
    assert_eq!(past(1, 0, 1), 3601000);
    assert_eq!(past(1, 0, 0), 3600000);
}
