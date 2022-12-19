
use std::{sync::{mpsc::{Sender, Receiver, channel, sync_channel, SyncSender}, Arc, Mutex}, thread, cell::RefCell};

#[test]
fn thread_join() {
    
}

#[test]
fn thread_barrier() {

}

#[test]
fn mutexes() {

}

#[test]
fn channels() {
    {
        let sx : Sender<String>;
        let rx : Receiver<String>;
        (sx, rx) = channel();

        sx.send(String::from("Hello world.")).expect("");
        assert_eq!("Hello world.", rx.recv().unwrap());    
        {
            // Senders can be cloned.
            let sx2 = Sender::clone(&sx);
            sx2.send(String::from("How are you?")).expect("");
        }
        assert_eq!("How are you?", rx.recv().unwrap());    

        // Senders can be passed into threads.
        let join_handle = thread::spawn(move || {
            sx.send(String::from("Good thanks.")).expect("");
        });
        assert_eq!("Good thanks.", rx.recv().unwrap());    
        join_handle.join().expect("");
    }
}

#[test]
fn sync_channels() {

    //rendezvous channels are used at triggers.
    {
        let sx : SyncSender<u8>;
        let rx : Receiver<u8>;
        (sx, rx) = sync_channel(0);
        
        let shared = Arc::new(Mutex::new(false));
        let shared2 = Arc::clone(&shared);

        // Senders can be passed into threads.
        let join_handle = thread::spawn(move || {
            rx.recv();
            let mut lock = shared2.lock().unwrap();
            assert_eq!(true, *lock);
            *lock = false;
        });

        {
            let mut lock = shared.lock().unwrap();
            assert_eq!(false, *lock);
            *lock = true;
        }

        sx.send(0).unwrap();
        join_handle.join().unwrap();
        
        let lock = shared.lock().unwrap();
        assert_eq!(false, *lock);
    }

}

#[test]
fn sync_and_send() {

}


