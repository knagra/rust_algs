use crate::print_time;

fn menu(arr: &[f64], target: f64) -> bool {
    let arr_len = arr.len();
    if arr_len == 0 { return false; }
    let next_item = arr[0];
    if target % next_item == 0.0 || (
        next_item < target && menu(&arr, target - next_item)
    ) { return true; }
    return menu(&arr[1..], target);
}

fn print_menu_run(arr: &[f64], target: f64) {
    println!("{}", menu(&arr, target));
}

pub fn run() {
    let arr: [f64; 6] = [2.15, 2.75, 3.35, 3.55, 4.2, 5.8];
    let prev_inst = print_time::print_time(None);
    print_menu_run(&arr, 15.05);
    let prev_inst = print_time::print_time(Some(prev_inst));
    print_menu_run(&arr, 20.0);
    let prev_inst = print_time::print_time(Some(prev_inst));
    print_menu_run(&arr, 10.47);
    let prev_inst = print_time::print_time(Some(prev_inst));
    print_menu_run(&arr, 11.47);
    let _ = print_time::print_time(Some(prev_inst));
}
