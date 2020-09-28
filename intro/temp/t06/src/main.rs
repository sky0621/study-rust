use std::sync::mpsc;
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut to_threads_senders = Vec::new();
    let mut to_main_receivers = Vec::new();

    // 10個のスレッド起動
    for _ in 0..10 {
        // main -> each thread
        let (to_threads_sender, to_threads_receiver) = mpsc::channel();
        // each thread -> main
        let (to_main_sender, to_main_receiver) = mpsc::channel();

        to_threads_senders.push(to_threads_sender);
        to_main_receivers.push(to_main_receiver);

        handles.push(thread::spawn(move || {
            // main -> each thread Receiver
            let mut data = to_threads_receiver.recv().unwrap();
            data += 1;
            // each thread -> main Sender
            let _ = to_main_sender.send(data);
        }));
    }

    // main -> each thread
    for x in 0..10 {
        let _ = to_threads_senders[x].send(data[x]);
    }

    // each thread -> main
    for x in 0..10 {
        data[x] = to_main_receivers[x].recv().unwrap();
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}

fn part1() {
    let (s, r) = mpsc::channel();
    let handle = thread::spawn(move || {
        let data = r.recv().unwrap();
        println!("{}", data);
    });

    let _ = s.send("Helloooooooooo");
    let _ = handle.join();
}
