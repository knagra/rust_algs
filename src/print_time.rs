extern crate time;

pub fn print_time(prev_sec: i64, prev_nsec: i32) -> (i64, i32) {
    let tm = time::get_time();
    if prev_sec == 0 {
        (tm.sec, tm.nsec)
    } else {
        let now: f64 = tm.sec as f64 - prev_sec as f64
            + ((tm.nsec as f64 - prev_nsec as f64) * 0.000000001 as f64);
        println!("{}", now);
        let tm = time::get_time();
        (tm.sec, tm.nsec)
    }
}
