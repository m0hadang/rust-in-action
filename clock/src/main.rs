use chrono::{DateTime, Days, Local, TimeZone};
use std::mem::zeroed;

#[cfg(not(windows))]
use libc;

#[cfg(not(windows))]
fn set_clock<Tz: TimeZone>(t: DateTime<Tz>) {
    use libc::{settimeofday, timezone};
    use libc::{suseconds_t, time_t, timeval};

    let t = t.with_timezone(&Local);
    let mut u: timeval = unsafe { zeroed() };

    u.tv_sec = t.timestamp() as time_t;
    u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

    unsafe {
        let mock_tz: *const timezone = std::ptr::null();
        settimeofday(&u as *const timeval, mock_tz);
    }
}

fn print_time() {
    let now = Local::now();
    println!("- unix time stamp : {}", now.timestamp());
    println!("- rfc2822(mail message header) : {}", now.to_rfc2822());
    println!("- rfc3339 : {}", now.to_rfc3339());
}

fn main() {
    println!("NOW");
    print_time();

    let now = Local::now();
    let now = now.checked_add_days(Days::new(3)).unwrap();
    set_clock(now);

    println!("AFTER 3 DAYS");
    print_time();
}
