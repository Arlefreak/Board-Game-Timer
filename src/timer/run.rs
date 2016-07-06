use std::time::Duration;
use std::thread::sleep;

pub fn run_timer(_time: i32){
    for x in 0.._time {
        println!("{}", x+1);
        sleep(Duration::from_millis(1000));
    }
}
