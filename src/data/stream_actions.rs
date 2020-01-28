use crossbeam_channel::{unbounded, Receiver, Sender};
use reqwest::{get, Url};
use std::error::Error;

pub struct StreamListener {
    uri: Url,
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
    /// # Returns - result of instance of StreamListener or dyn Error otherwise 
    /// 
    pub fn new(uri: String) -> Result<Self, Box<dyn Error>> {
        let (sender, receiver) = unbounded();
        let uri = Url::parse(&uri)?;
        Ok(Self { uri, receiver, sender })
    }

    /// Getter for receiver
    /// 
    /// # Returns - receiver pipe that listen for decoded stream chunk
    /// 
    pub fn get_listener(&self) -> Receiver<Vec<f32>> {
        self.receiver.clone()
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        let response = get(self.uri.clone()).await?.text().await?;
        println!("{:?}", response);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::StreamListener;
    use futures_await_test::async_test;
    use tokio::runtime::Runtime;

    #[async_test]
    async fn test_get_stream() {
        let mut listener = StreamListener::new(format!("https://zt01.cdn.eurozet.pl/zet-net.mp3")).unwrap();
        let a = Runtime::new().unwrap().block_on(listener.run());
    }
}