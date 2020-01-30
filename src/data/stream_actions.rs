use super::{decode_mp3_from_chunk, PlaylistHelper};
use bytes::{Buf, Bytes};
use crossbeam_channel::{unbounded, Receiver, Sender};
use m3u8_rs::playlist::{MasterPlaylist, MediaPlaylist, Playlist, VariantStream};
use reqwest::{get, Url};
use std::error::Error;
use std::io::{Error as ErrorIo, ErrorKind, Read};
use std::mem::drop;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use tokio::runtime::Runtime;

struct ReadableBuffer {
    data: Vec<u8>,
    bytes_read: usize,
}

#[derive(Clone, Debug)]
struct StreamListener {
    uri: Url,
    receiver: Receiver<Vec<f32>>,
    sender: Sender<Vec<f32>>,
    is_active: bool,
}

#[derive(Clone, Debug)]
pub struct ArcStreamListener(Arc<Mutex<StreamListener>>);

impl Read for ReadableBuffer {
    // TODO, docs
    //
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ErrorIo> {
        if self.data.len() == 0 || self.bytes_read == self.data.len() {
            drop(self);
            return Err(ErrorIo::from(ErrorKind::InvalidData));
        } else if buf.len() == 0 {
            return Err(ErrorIo::from(ErrorKind::InvalidInput));
        }
        let mut _read: usize = 0;
        loop {
            if _read < buf.len() && self.bytes_read < self.data.len() {
                let bite = self.data[self.bytes_read];
                buf[_read] = bite;
                _read += 1;
                self.bytes_read += 1;
                if self.bytes_read == self.data.len() {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(_read)
    }
}

impl ReadableBuffer {
    fn from_bytes(bytes_buf: &Bytes) -> Self {
        let mut data = Vec::new();
        let bytes_read = 0;
        bytes_buf.bytes().into_iter().for_each(|bite| {
            data.push(*bite);
        });
        Self { data, bytes_read }
    }
}

#[allow(dead_code)]
impl ArcStreamListener {
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
        let is_active = false;
        Ok(Self(Arc::new(Mutex::new(StreamListener {
            uri,
            receiver,
            sender,
            is_active,
        }))))
    }

    /// Getter for receiver
    ///
    /// # Returns receiver pipe that listen for decoded stream chunk
    ///
    pub fn get_listener(&self) -> Receiver<Vec<f32>> {
        self.0.lock().unwrap().receiver.clone()
    }

    /// Check if stream listener is active
    ///
    /// # Returns true if listener is in active state, false otherwise
    pub fn is_active(&self) -> bool {
        self.0.lock().unwrap().is_active
    }

    /// Deactivates stream listener
    pub fn deactivate(&mut self) {
        self.0.lock().unwrap().is_active = false;
    }

    /// Runs loop for fetching m3u8 stream
    ///
    /// # Returns Ok if endpoint responded with valid playlist and stream or dyn Error otherwise
    ///
    pub async fn run_m3u8(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        let master_playlist = fetch_master_playlist(&self.0.lock().unwrap().uri).await?;
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
        if self.0.lock().unwrap().is_active == true {
            return Err(Box::from(format!(
                "Listener is active and should be deactivated first"
            )));
        }
        self.0.lock().unwrap().is_active = true;
        let listener_clone = self.clone();
        let stream_listener_proc = thread::spawn(move || {
            if let Err(e) = Runtime::new()
                .unwrap()
                .block_on(listen_mp3_stream(listener_clone))
            {
                panic!(format!("{:?}", e));
            };
        });
        Ok(stream_listener_proc)
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

async fn listen_mp3_stream(listener: ArcStreamListener) -> Result<(), Box<dyn Error + 'static>> {
    let mut res = reqwest::get(listener.0.lock().unwrap().uri.clone()).await?;
    while let Some(chunk) = res.chunk().await? {
        if listener.0.lock().unwrap().is_active == false {
            // stop the process and exit
            println!("stop");
            process::exit(0x0100);
        };
        // println!("Chunk: {:?}", chunk);
        let readable_buffer = ReadableBuffer::from_bytes(&chunk);
        let mut arr: Vec<u8> = vec![0; readable_buffer.data.len()];
        // TODO: resolve reading from buffer
        // let decoded = decode_mp3_from_chunk();
        // println!("\n Decoded {:?} \n", &decoded);
        // sender.send()
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
    use super::{Runtime, ReadableBuffer, ArcStreamListener, Read};
    use futures_await_test::async_test;
    use std::thread::sleep;
    use std::time::Duration;

    #[async_test]
    async fn test_get_m3u8_stream() {
        let mut listener = ArcStreamListener::new(
            format!("http://a.files.bbci.co.uk/media/live/manifesto/audio/simulcast/hls/uk/sbr_high/ak/bbc_radio_two.m3u8")
        ).unwrap();
        let a = Runtime::new().unwrap().block_on(listener.run_m3u8());
        println!("{:?}", &a);
    }
    #[async_test]
    async fn test_get_mp3_stream() {
        let mut listener = ArcStreamListener::new(
            format!("https://str2b.openstream.co/604?aw_0_1st.collectionid=3162&stationId=3162&publisherId=628&listenerid=1580311050432_0.47836979431904714&awparams=companionAds%3Atrue&aw_0_1st.version=1.1.4%3Ahtml5")
        ).unwrap();
        if let Ok(a) = Runtime::new().unwrap().block_on(listener.run_mp3()) {
            println!("{:?}", &a);
            sleep(Duration::from_secs(5));
            listener.deactivate();
            a.join().unwrap();
        };
    }

    #[test]
    fn test_read_from_buffer() {
        let data: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10,11];
        let bytes_read = 0;
        let mut buf_read = ReadableBuffer { data, bytes_read };
        let mut buf_write: [u8; 2] = [0; 2];
        let res = buf_read.read(&mut buf_write);
        println!("--- Read from buffer test info, look in to file to get head around cases... ---");
        println!("{:?}", &res);
        println!("{:?}", &buf_write);
        assert_eq!(res.unwrap(), 2);
        assert_eq!(buf_write[0], 1);
        assert_eq!(buf_write[1], 2);
        let res = buf_read.read(&mut buf_write);
        println!("{:?}", &res);
        println!("{:?}", &buf_write);
        assert_eq!(res.unwrap(), 2);
        assert_eq!(buf_write[0], 3);
        assert_eq!(buf_write[1], 4);
        let res = buf_read.read(&mut buf_write);
        println!("{:?}", &res);
        println!("{:?}", &buf_write);
        assert_eq!(res.unwrap(), 2);
        assert_eq!(buf_write[0], 5);
        assert_eq!(buf_write[1], 6);
        let res = buf_read.read(&mut buf_write);
        println!("{:?}", &res);
        println!("{:?}", &buf_write);
        assert_eq!(res.unwrap(), 2);
        assert_eq!(buf_write[0], 7);
        assert_eq!(buf_write[1], 8);
        let res = buf_read.read(&mut buf_write);
        println!("{:?}", &res);
        println!("{:?}", &buf_write);
        assert_eq!(res.unwrap(), 2);
        assert_eq!(buf_write[0], 9);
        assert_eq!(buf_write[1], 10);
        let res = buf_read.read(&mut buf_write);
        println!("{:?}", &res);
        println!("{:?}", &buf_write);
        assert_eq!(res.unwrap(), 1);
        assert_eq!(buf_write[0], 11);
        assert_eq!(buf_write[1], 10);
        let res = buf_read.read(&mut buf_write);
        println!("{:?}", &res);
        println!("{:?}", &buf_write);
        if let Err(_) = res {
            assert_eq!(1, 1);
        } else {
            assert_eq!(1, 2);
        }
        assert_eq!(buf_write[0], 11);
        assert_eq!(buf_write[1], 10);
        // test vector
        let data: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10,11];
        let bytes_read = 0;
        let mut buf_read = ReadableBuffer { data, bytes_read };
        let mut buf_write: Vec<u8> = vec![0; buf_read.data.len()];
        let res = buf_read.read(&mut buf_write);
        println!("{:?}", &res);
        println!("{:?}", &buf_write);
        assert_eq!(res.unwrap(), 11);
        assert_eq!(buf_write[0], 1);
        assert_eq!(buf_write[10], 11);
        println!("--- Read buffer test finished ---");
    }
}
