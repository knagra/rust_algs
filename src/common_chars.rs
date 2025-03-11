use crate::print_time;

fn common_chars(arr: &Vec<String>) -> (String, usize) {
    let mut common = String::from("");
    'outer: for character in arr[0].chars() {
        if common.contains(character) { continue; }
        for x in 1..arr.len() {
            if !(arr[x].contains(character)) { continue 'outer; }
        }
        common.push(character);
    }
    let l: usize = common.len();
    return (common, l);
}

pub fn run() {
    let arr1 = vec![String::from("abcdefga"), String::from("xvxaa"), String::from("bcdefa")];
    let arr2 = vec![String::from("abcdefgw"), String::from("abcuvw"), String::from("xvwcba")];
    let (common, num): (String, usize) = common_chars(&arr1);
    let prev_inst = print_time::print_time(None);
    println!("Common chars: {}. Number of chars: {}.", common, num);
    let prev_inst = print_time::print_time(Some(prev_inst));
    let (common, num) = common_chars(&arr2);
    println!("Common chars: {}. Number of chars: {}.", common, num);
    let _ = print_time::print_time(Some(prev_inst));
}
