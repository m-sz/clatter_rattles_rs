use super::PlaylistHelper;
use crossbeam_channel::{unbounded, Receiver, Sender};
use m3u8_rs::playlist::{MasterPlaylist, MediaPlaylist, Playlist, VariantStream};
use reqwest::{get, Url};
use std::error::Error;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use tokio::runtime::Runtime;

#[derive(Clone, Debug)]
pub struct StreamListener {
    uri: Url,
    receiver: Receiver<Vec<f32>>,
    sender: Sender<Vec<f32>>,
    is_active: Arc<Mutex<bool>>,
}

#[allow(dead_code)]
impl StreamListener {
    /// Create new instance of StreamListener
    ///
    /// # Arguments:
    ///
    /// * uri - address of external server emitting stream
    ///
    /// # Returns result of instance of StreamListener or dyn Error otherwise
    ///
    pub fn new(uri: String) -> Result<Self, Box<dyn Error>> {
        let (sender, receiver) = unbounded();
        let uri = Url::parse(&uri)?;
        let is_active = Arc::new(Mutex::new(false));
        Ok(Self {
            uri,
            receiver,
            sender,
            is_active,
        })
    }

    /// Getter for receiver
    ///
    /// # Returns receiver pipe that listen for decoded stream chunk
    ///
    pub fn get_listener(&self) -> Receiver<Vec<f32>> {
        self.receiver.clone()
    }

    /// Check if stream listener is active
    ///
    /// # Returns true if listener is in active state, false otherwise
    pub fn is_active(&self) -> bool {
        self.is_active.lock().unwrap().clone()
    }

    /// Deactivates stream listener
    pub fn deactivate(&mut self) {
        *self.is_active.lock().unwrap() = false;
    }

    /// Runs loop for fetching m3u8 stream
    ///
    /// # Returns Ok if endpoint responded with valid playlist and stream or dyn Error otherwise
    ///
    pub async fn run_m3u8(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        let master_playlist = fetch_master_playlist(&self.uri).await?;
        if let Ok(uri) = master_playlist.find_uri() {
            let uri = Url::parse(&uri)?;
            let media_playlist = fetch_media_playlist(&uri).await?;
        };
        // TODO: need implementation of loop of stream read
        Ok(())
    }

    /// Runs listener ins separate thread that collects stream and feeds pipe sender
    ///
    /// # Returns success of thread join handle if listener is not active and async block has no errors, dyn Error otherwise
    /// 
    pub async fn run_mp3(&self) -> Result<JoinHandle<()>, Box<dyn Error>> {
        if *self.is_active.lock().unwrap() == true {
            return Err(Box::from(format!(
                "Listener is active and should be deactivated first"
            )));
        }
        let uri = self.uri.clone();
        let sender = self.sender.clone();
        let is_active = self.is_active.clone();
        let listener = thread::spawn(move || {
            if let Err(e) = Runtime::new()
                .unwrap()
                .block_on(listen_mp3_stream(uri, sender, is_active))
            {
                panic!(format!("{:?}", e));
            };
        });
        Ok(listener)
    }
}

impl PlaylistHelper for MasterPlaylist {
    fn find_uri(&self) -> Result<String, Box<dyn Error>> {
        let variants: Vec<VariantStream> = self
            .variants
            .clone()
            .into_iter()
            .filter(|variant_stream| variant_stream.uri.len() > 0)
            .collect();
        if variants.len() > 0 {
            return Ok(variants[0].uri.to_owned());
        }
        Err(Box::from(format!(
            "No available uri in MasterPlaylist variants"
        )))
    }
}

async fn listen_mp3_stream(
    uri: Url,
    sender: Sender<Vec<f32>>,
    is_active: Arc<Mutex<bool>>,
) -> Result<(), Box<dyn Error + 'static>> {
    let mut res = reqwest::get(uri).await?;
    while let Some(chunk) = res.chunk().await? {
        if *is_active.lock().unwrap() == false {
            // stop the process and exit
            process::exit(0x0100);
        };
        println!("Chunk: {:?}", chunk);
    }
    Ok(())
}

async fn fetch_master_playlist(uri: &Url) -> Result<MasterPlaylist, Box<dyn Error + 'static>> {
    let text = get_from_as_string(uri).await?;
    match m3u8_rs::parse_playlist_res(&text.as_bytes()) {
        Ok(Playlist::MasterPlaylist(pl)) => Ok(pl),
        Ok(Playlist::MediaPlaylist(_)) => Err(Box::from(format!("Wrong format of media playlist"))),
        Err(e) => Err(Box::from(format!("{:?}", e))),
    }
}

async fn fetch_media_playlist(uri: &Url) -> Result<MediaPlaylist, Box<dyn Error + 'static>> {
    let text = get_from_as_string(uri).await?;
    match m3u8_rs::parse_playlist_res(&text.as_bytes()) {
        Ok(Playlist::MasterPlaylist(_)) => {
            Err(Box::from(format!("Wrong format of media playlist")))
        }
        Ok(Playlist::MediaPlaylist(pl)) => Ok(pl),
        Err(e) => Err(Box::from(format!("{:?}", e))),
    }
}

async fn get_from_as_string(uri: &Url) -> Result<String, Box<dyn Error + 'static>> {
    let text = get(uri.clone()).await?.text().await?;
    Ok(text)
}

#[cfg(test)]
mod test {
    use super::Runtime;
    use super::StreamListener;
    use futures_await_test::async_test;

    #[async_test]
    async fn test_get_m3u8_stream() {
        let mut listener = StreamListener::new(
            format!("http://a.files.bbci.co.uk/media/live/manifesto/audio/simulcast/hls/uk/sbr_high/ak/bbc_radio_two.m3u8")
        ).unwrap();
        let a = Runtime::new().unwrap().block_on(listener.run_m3u8());
        println!("{:?}", &a);
    }
    #[async_test]
    async fn test_get_mp3_stream() {
        let mut listener = StreamListener::new(
            format!("https://str2b.openstream.co/604?aw_0_1st.collectionid=3162&stationId=3162&publisherId=628&listenerid=1580311050432_0.47836979431904714&awparams=companionAds%3Atrue&aw_0_1st.version=1.1.4%3Ahtml5")
        ).unwrap();
        if let Ok(a) = Runtime::new().unwrap().block_on(listener.run_mp3()) {
            println!("{:?}", &a);
            a.join().unwrap();
        };
    }
}
