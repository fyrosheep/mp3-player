use std::{fs::File, io::BufReader, path::PathBuf};

use clap::Parser;
use rand::{thread_rng, Rng};
use rodio::{Decoder, OutputStream, Sink};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    filename: String,
    #[clap(short, long, value_parser)]
    shuffle: bool,
    #[clap(short, long, value_parser)]
    repeat: bool,
    #[clap(short, long, value_parser)]
    quiet: bool,
}

fn is_music_file(filename: &PathBuf) -> bool {
    match filename.extension() {
        Some(ext) => match ext.to_str() {
            Some(ext) => match ext {
                "mp3" => true,
                "wav" => true,
                "ogg" => true,
                _ => false,
            },
            None => false,
        },
        None => false,
    }
}

fn get_all_music_files(filename: PathBuf) -> Vec<PathBuf> {
    let mut files = vec![];
    if filename.is_dir() {
        for file in filename.read_dir().unwrap() {
            let file = file.unwrap();
            if is_music_file(&file.path()) {
                files.push(file.path());
            }
        }
    } else {
        if is_music_file(&filename) {
            files.push(filename);
        }
    }
    files
}

fn pick_music_file(music_files: &Vec<PathBuf>, args: &Args) -> PathBuf {
    if args.shuffle {
        let index = thread_rng().gen_range(0..music_files.len());
        return music_files[index].clone();
    }
    return music_files[0].clone();
}

// TODO: hookup to keyboard shortcuts

fn main() {
    let args = Args::parse();

    let filename = args.filename.clone().into();
    let mut music_files = get_all_music_files(filename);

    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    loop {
        if music_files.len() == 0 {
            if !args.quiet {
                println!("Our playlist is now over!");
            }
            break;
        }

        let path = pick_music_file(&music_files, &args);
        let file = BufReader::new(File::open(&path).unwrap());
        let source = Decoder::new(file).unwrap();
        
        if !args.quiet {
            println!("Now playing: {}", path.display());
        }

        sink.append(source);
        sink.sleep_until_end();
        if !args.repeat {
            let index = music_files.iter().position(|x| *x == path).unwrap();
            music_files.remove(index);
        }
    }
}
