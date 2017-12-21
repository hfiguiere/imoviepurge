

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate walkdir;

use std::path::{Path, PathBuf};

use docopt::Docopt;
use walkdir::WalkDir;

const USAGE: &'static str = "
iMovie purge.

Usage:
  imoviepurge -l <imovielibrary> -d <source>
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_l: bool,
    arg_imovielibrary: String,
    flag_d: bool,
    arg_source: String,
}

const MEDIA_SUBDIR: &'static str = "Original Media";


fn list_media_assets(library: &Path) -> Vec<PathBuf> {
    let mut assets: Vec<PathBuf> = vec!();

    for entry in library.read_dir().expect("path not found") {
        if !entry.is_ok() {
            continue;
        }
        let entry = entry.unwrap();
        let mut path = entry.path();
        if !path.is_dir() {
            continue;
        }
        path.push(MEDIA_SUBDIR);
        if !path.exists() {
            continue;
        }
        for media in path.read_dir().expect("media directory error") {
            if !media.is_ok() {
                continue;
            }
            assets.push(media.unwrap().path());
        }
    }

    assets
}

fn list_source_assets(source: &Path) -> Vec<PathBuf> {
    let mut assets: Vec<PathBuf> = WalkDir::new(source)
        .into_iter()
        .filter_map(|e| {
            if let Ok(e) = e {
                let path = e.path();
                if path.is_file() {
                    return Some(PathBuf::from(path));
                }
            }
            None
        })
        .collect();

    assets
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let library = Path::new(&args.arg_imovielibrary);
    let assets = list_media_assets(library);
//    println!("assets: {:?}", assets);

    let source = Path::new(&args.arg_source);
    let source_assets = list_source_assets(source);
//    println!("source assets: {:?}", source_assets);
}
