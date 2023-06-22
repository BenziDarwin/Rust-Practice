//! # main
//! This crate holds an example fo threads, how to instanciate them and use them.

use std::thread;
use  std::time::Duration;
use std::sync::mpsc;

/// This function uses threads and also going to be used for message passing.

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();
    //Sender has to be cloned so that multiple threads can use the send method.
    let tx1 = tx.clone();
    let trd = thread::spawn(move || {
        let arr2 = vec![1,2,3,4,5,6,7,8];
        for i in arr2 {
            tx1.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Using the move key word moves variables from the main scope into the thread.
    let val = vec![1,2,3,4,5];
    let trd1 = thread::spawn(move || {
        println!("{:?}",val);
        for i in val {
            tx.clone().send(i).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    let arr1 = vec![1,2,3,4,5];
    for i in arr1 {
        println!("{i}");
        thread::sleep(Duration::from_millis(1));
        println!("This is {}",rx.recv().unwrap_or_else(|err| { 
            println!("{}",err); 
            0}));
    }
    // Where join is called from will affect how each thread is run in the whole program.
    trd.join().unwrap();
    trd1.join().unwrap();
}
