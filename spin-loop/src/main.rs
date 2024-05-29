use std::{thread, time};
fn spin_loop() {
    for n in 1..1001 {
        let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);

        let start = time::Instant::now();
        for _m in 0..n {
            let handle = thread::spawn(|| {
                let start = time::Instant::now();
                let pause = time::Duration::from_millis(20);
                while start.elapsed() < pause {
                    thread::yield_now();
                }
                /*
                 * - sleep version
                 * thread::sleep(time::Duration::from_millis(20))
                 * */
            });
            handlers.push(handle);
        }

        // for handle in &handlers {
        //     // !! compile error : thread is no more exists
        //     // can't refrence not exists(terminated thread) ->
        //     // should not access another thread
        //     // if you wnat fix compile error then remove & from handlers
        //     handle.join();
        // }

        while let Some(handle) = handlers.pop() {
            handle.join();
        }

        let finish = time::Instant::now();
        println!("{}\t{:02?}", n, finish.duration_since(start));
    }
}
fn main() {
    spin_loop();
}
