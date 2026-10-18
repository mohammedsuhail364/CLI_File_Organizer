use std::{
    ops::AddAssign,
    sync::Mutex,
    thread::{scope, sleep},
    time::Duration,
};

pub fn test_mutex() {
    let score = Mutex::new(0u16);
    // let unlocked_data = score.lock();
    // let mut data = unlocked_data.unwrap();
    // data.add_assign(5);
    // println!("{}",data);
    // drop(data);
    let myfunc = || {
        println!("Thread 1 is waiting for mutex lock");

        let mut data = score.lock().unwrap();
        for i in 1..10 {
            data.add_assign(i);
            println!("Thread 1 is adding {i}");
            sleep(Duration::from_millis(900));
        }
    };
    let myfunc2 = || {
        loop {
            println!("Thread 2 is waiting for mutex lock");
            let guard = score.try_lock();
            if guard.is_ok() {
                let mut data = guard.unwrap();
                for i in 1..10 {
                    data.add_assign(i);
                    println!("Thread 2 is adding {i}");
                }
                break;
            }
            sleep(Duration::from_millis(400));
        }
    };
    scope(|s| {
        s.spawn(myfunc);
        s.spawn(myfunc2);
    });
    println!("{:?}", score.lock().unwrap());
}
