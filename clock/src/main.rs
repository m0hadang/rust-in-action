use chrono::Local;

fn main() {
    let now = Local::now();
    println!("unix time stamp : {}", now.timestamp());
    println!("rfc2822(mail message header) : {}", now.to_rfc2822());
    println!("rfc3339 : {}", now.to_rfc3339());
}
