use super::PlaylistHelper;
use crossbeam_channel::{unbounded, Receiver, Sender};
use m3u8_rs::playlist::{MasterPlaylist, MediaPlaylist, Playlist, VariantStream};
use reqwest::{get, Url};
use std::error::Error;

pub struct StreamListener {
    uri: Url,
    receiver: Receiver<Vec<f32>>,
    sender: Sender<Vec<f32>>,
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
        Ok(Self {
            uri,
            receiver,
            sender,
        })
    }

    /// Getter for receiver
    ///
    /// # Returns - receiver pipe that listen for decoded stream chunk
    ///
    pub fn get_listener(&self) -> Receiver<Vec<f32>> {
        self.receiver.clone()
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        let master_playlist = fetch_master_playlist(&self.uri).await?;
        println!("{:?}", &master_playlist);
        if let Ok(uri) = master_playlist.find_uri() {
            let uri = Url::parse(&uri)?;
            let media_playlist = fetch_media_playlist(&uri).await?;
            println!("{:?}", &media_playlist);
        };
        Ok(())
    }
}

impl PlaylistHelper<MasterPlaylist> for MasterPlaylist {
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
        Err(Box::from(format!("No available uri in MasterPlaylist variants")))
    }
}

async fn fetch_master_playlist(uri: &Url) -> Result<MasterPlaylist, Box<dyn Error + 'static>> {
    let text = get_from(uri).await?;
    match m3u8_rs::parse_playlist_res(&text.as_bytes()) {
        Ok(Playlist::MasterPlaylist(pl)) => Ok(pl),
        Ok(Playlist::MediaPlaylist(_)) => Err(Box::from(format!("Wrong format of media playlist"))),
        Err(e) => Err(Box::from(format!("{:?}", e))),
    }
}

async fn fetch_media_playlist(uri: &Url) -> Result<MediaPlaylist, Box<dyn Error + 'static>> {
    let text = get_from(uri).await?;
    match m3u8_rs::parse_playlist_res(&text.as_bytes()) {
        Ok(Playlist::MasterPlaylist(_)) => {
            Err(Box::from(format!("Wrong format of media playlist")))
        }
        Ok(Playlist::MediaPlaylist(pl)) => Ok(pl),
        Err(e) => Err(Box::from(format!("{:?}", e))),
    }
}

async fn get_from(uri: &Url) -> Result<String, Box<dyn Error + 'static>> {
    let text = get(uri.clone()).await?.text().await?;
    Ok(text)
}

#[cfg(test)]
mod test {
    use super::StreamListener;
    use futures_await_test::async_test;
    use tokio::runtime::Runtime;

    #[async_test]
    async fn test_get_stream() {
        let mut listener = StreamListener::new(
            format!("http://a.files.bbci.co.uk/media/live/manifesto/audio/simulcast/hls/uk/sbr_high/ak/bbc_radio_two.m3u8")
        ).unwrap();
        let a = Runtime::new().unwrap().block_on(listener.run());
        println!("{:?}", &a);
    }
}
