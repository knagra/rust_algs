use crate::print_time;

fn kadane(arr: &[f64]) -> (f64, usize, usize) {
    if arr.len() == 0 {
        panic!("Empty array");
    }
    let mut max_to_here: f64 = arr[0];
    let mut max_so_far: f64 = arr[0];
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut potential_start: usize = 0;
    for x in 0..arr.len() {
        if arr[x] > max_to_here + arr[x] {
            potential_start = x;
            max_to_here = arr[x];
        } else {
            max_to_here = arr[x] + max_to_here;
        }
        if max_to_here > max_so_far {
            end = x;
            start = potential_start;
            max_so_far = max_to_here;
        }
    }
    (max_so_far, start, end)
}

pub fn run() {
    let arr1: [f64; 4] = [-2.0, -3.0, -4.0, -1.0];
    let arr2: [f64; 10] = [-2.0, -3.0, 4.0, -1.0, -2.0, 1.0, 5.0, -3.0, 2.0, 1.0];
    let prev_inst = print_time::print_time(None);
    let (max_so_far, start, end) = kadane(&arr1);
    println!("{}, {}, {}", max_so_far, start, end);
    let prev_inst = print_time::print_time(Some(prev_inst));
    let (max_so_far, start, end) = kadane(&arr2);
    println!("{}, {}, {}", max_so_far, start, end);
    let _ = print_time::print_time(Some(prev_inst));
}
