use std::time::Instant;

pub fn print_duration(func: fn()) {
    let start = Instant::now();
    func();
    let duration = start.elapsed();
    println!("{:?}", duration);
}
