use print_time;

fn common_chars(arr: &Vec<str>) -> (str, u64) {
    let mut common = "";
    'outer: for character in arr[0].chars() {
        if common.contains(character) { continue; }
        'inner: for x in 1..arr.len() {
            if !(arr[x].contains(character)) { continue 'outer; }
        }
        common_chars += character;
    }
    return (common_chars, common_chars.len());
}

pub fn run() {
    let arr1 = vec!["abcdefga", "xvxaa", "bcdefa"];
    let arr2 = vec!["abcdefgw", "abcuvw", "xvwcba"];
    let (mut common, mut num): (str, u64) = common_chars(&arr1);
    let (sec, nsec) = print_time::print_time(0 as i64, 0 as i32);
    println!("Common chars: {}. Number of chars: {}.", common, num);
    let (sec, nsec) = print_time::print_time(sec, nsec);
    (common, num) = common_chars(&arr2);
    println!("Common chars: {}. Number of chars: {}.", common, num);
    let (_, _) = print_time::print_time(sec, nsec);
}
