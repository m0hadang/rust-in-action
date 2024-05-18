use chrono::{DateTime, Days, Local, TimeZone};
use std::mem::zeroed;

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

#[cfg(windows)]
fn set_clock<Tz: TimeZone>(t: DateTime<Tz>) -> () {
    use chrono::Weekday;
    use chrono::{Datelike, Timelike};
    use kernel32::SetSystemTime;
    use winapi::{SYSTEMTIME, WORD};

    let t = t.with_timezone(&Local);

    let mut systime: SYSTEMTIME = unsafe { zeroed() };

    let dow = match t.weekday() {
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
        Weekday::Sun => 0,
    };

    let mut ns = t.nanosecond();
    let is_leap_second = ns > 1_000_000_000;

    if is_leap_second {
        ns -= 1_000_000_000;
    }

    systime.wYear = t.year() as WORD;
    systime.wMonth = t.month() as WORD;
    systime.wDayOfWeek = dow as WORD;
    systime.wDay = t.day() as WORD;
    systime.wHour = t.hour() as WORD;
    systime.wMinute = t.minute() as WORD;
    systime.wSecond = t.second() as WORD;
    systime.wMilliseconds = (ns / 1_000_000) as WORD;

    let system_time_ptr: *const SYSTEMTIME = &systime as *const SYSTEMTIME;

    unsafe {
        SetSystemTime(system_time_ptr);
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
