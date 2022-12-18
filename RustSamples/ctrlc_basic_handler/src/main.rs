use std::sync::{Arc, atomic::AtomicBool};
use core::sync::atomic::Ordering::Relaxed;
use std::{thread, time};

fn main() {
    let ctrlc_flag : Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let ctrlc_flag_2 = ctrlc_flag.clone();

    ctrlc::set_handler(move || {
        println!("Stop Requested.");
        ctrlc_flag.store(true, Relaxed);
    }).expect("Failed to set ctrlc handler.");

    loop {
        if ctrlc_flag_2.load(Relaxed) == true 
        {
            println!("Stopping.");
            thread::sleep(time::Duration::from_millis(5000));
            std::process::exit(0);
        }
        println!("Looping.");
        thread::sleep(time::Duration::from_millis(500));
    }
}