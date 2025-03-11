use std::time::Instant;

pub fn print_time(prev_inst: Option<Instant>) -> Instant {
    let now: Instant = Instant::now();
    if prev_inst == None {
        now
    } else {
        println!("{:?}", now.duration_since(prev_inst.unwrap()));
        Instant::now()
    }
}
