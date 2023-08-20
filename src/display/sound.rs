use std::fs::File;
use std::io::BufReader;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use rodio::{Decoder, OutputStream, Sink};
use walkdir::WalkDir;

type MusicSource = Decoder<BufReader<File>>;

pub struct Sound
{
    music_path: String,
    _stream: OutputStream,
    sink: Sink,
}

impl Sound
{
    pub fn new(music_path: &String) -> Result<Sound, String>
    {
        let (stream, stream_handle) = match rodio::OutputStream::try_default()
        {
            Ok((stream, handle)) => (stream, handle),
            Err(_) =>
            {
                let err_msg = "Could not create music stream.".to_string();
                return Err(err_msg);
            }
        };

        let sink = match Sink::try_new(&stream_handle)
        {
            Ok(s) => s,
            Err(_) =>
            {
                let err_msg = "Could not create music sink.".to_string();
                return Err(err_msg);
            }
        };

        Self::attach_sources(&sink, music_path);

        let sound = Sound
        {
            music_path: music_path.clone(),
            _stream: stream,
            sink,
        };

        Ok(sound)
    }

    fn get_music_sources(path: &String) -> Vec<MusicSource>
    {
        let mut sources = vec![];

        let dir = WalkDir::new(path);
        for entry in dir.into_iter().skip(1)
        {
            let entry = entry.unwrap();
            let name = entry.path().display().to_string();

            let file = File::open(name).unwrap();
            let source = rodio::Decoder::new(BufReader::new(file));

            match source
            {
                Ok(s) => sources.push(s),
                Err(_) => {},
            }
        }

        let mut rng = thread_rng();
        sources.shuffle(&mut rng);

        sources
    }

    fn attach_sources(sink: &Sink, path: &String)
    {
        let sources = Self::get_music_sources(path);
        for source in sources
        {
            sink.append(source);
        }
    }

    pub fn play(&mut self)
    {
        if self.sink.empty()
        {
            Self::attach_sources(&self.sink, &self.music_path);
        }
    }
}
