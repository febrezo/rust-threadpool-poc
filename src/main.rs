use std::{thread, time};
use threadpool::ThreadPool;

fn execute_task(task_id: i32) {
    println!("THREAD > Starting #{}...", task_id.to_string());

    let nap = time::Duration::from_millis(1000);
    thread::sleep(nap);
    println!("THREAD > End of #{}.", task_id.to_string());
}

fn main() {
    println!("MAIN > Starting...");

    let pool = ThreadPool::new(4);

    for n in 1..=20 {
        println!("MAIN > Preparing execution #{}", n.to_string());

        pool.execute(move || {
            execute_task(n);
        });
    }

    pool.join();
    println!("MAIN > Ended.");
}
