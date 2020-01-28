use crossbeam_channel::{unbounded, Receiver, Sender};
// use gst;

pub struct StreamListener {
    uri: String,
    receiver: Receiver<Vec<f32>>,
    sender: Sender<Vec<f32>>
}

#[allow(dead_code)]
impl StreamListener {
    /// Create new instance of StreamListener
    /// 
    /// # Arguments:
    /// 
    /// * uri - address of external server emitting stream
    /// 
    /// # Returns - instance of StreamListener
    /// 
    pub fn new(uri: String) -> Self {
        let (sender, receiver) = unbounded();
        Self { uri, receiver, sender }
    }

    /// Getter for receiver
    /// 
    /// # Returns - receiver pipe that listen for decoded stream chunk
    /// 
    pub fn get_listener(&self) -> Receiver<Vec<f32>> {
        self.receiver.clone()
    }

    pub fn run(&mut self) {
        unimplemented!();
    }
}

