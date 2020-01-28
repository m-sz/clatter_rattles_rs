
extern crate use crossbeam_channel::{unbounded, Receiver, Sender};
extern crate gst;

pub struct StreamListener {
    uri: &str,
    receiver: Receiver,
    sender: Sender
}

impl StreamListener {
    pub fn new(uri: &str) -> Self {
        let (sender, receiver) = unbounded();
        Self { uri, receiver, sender }
    }
}

