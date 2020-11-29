use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn thread_spawn_test() {
    let v = vec![1, 2, 3];
    // add move keyword to borrow v from main thread to sub-thread
    let handle = thread::spawn(move || {
        println!("Here is a vector: {:?}", v);
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // drop(v);

    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn chanel_message_com_test() {
    // channel message com bt multi-thread
    let (tx, rx) = mpsc::channel();

    // multi-producer by clone tx
    let tx1 = mpsc::Sender::clone(&tx);

    // thread with tx1
    thread::spawn(move || {
        // let val = String::from("hi");
        // tx.send(val).unwrap(); // send takes ownership of its parameter and the value is moved
        // println!("Sent: {}", val);

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    // thread with tx
    thread::spawn(move || {
        // let val = String::from("hi");
        // tx.send(val).unwrap(); // send takes ownership of its parameter and the value is moved
        // println!("Sent: {}", val);

        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
    for received in rx {
        println!("Got: {}", received);
    }
}

// use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn mutex_test() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}

fn mutex_com_test() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let cnt = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = cnt.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result : {}", *counter.lock().unwrap());
}

fn main() {
    thread_spawn_test();
    chanel_message_com_test();
    mutex_test();
    mutex_com_test();
}
