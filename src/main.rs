

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate walkdir;

use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
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

    for media_dir in library.read_dir()
        .expect("path not found")
        .filter_map(|entry| {
            if !entry.is_ok() {
                return None;
            }
            let entry = entry.unwrap();
            let mut path = entry.path();
            if !path.is_dir() {
                return None;
            }
            path.push(MEDIA_SUBDIR);
            if path.exists() {
                return Some(path);
            }
            None
        }) {
            for media in media_dir.read_dir().expect("media directory error") {
                if !media.is_ok() {
                    continue;
                }
                assets.push(media.unwrap().path());
            }
        }

    assets
}

fn list_source_assets(source: &Path) -> Vec<PathBuf> {
    let assets: Vec<PathBuf> = WalkDir::new(source)
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

fn intersect(set: Vec<PathBuf>, source: Vec<PathBuf>) -> Vec<PathBuf> {
    let lib_content: HashMap<OsString, PathBuf> = set.into_iter()
        .filter_map(|asset| {
            if let Some(file_name) = asset.file_name() {
                return Some((file_name.to_os_string(),
                             asset.clone()));
            }
            None
        })
        .collect();

    return source.into_iter()
        .filter_map(|source_media| {
            if let Some(file_name) = source_media.file_name() {
                let file_name = file_name.to_os_string();
                if let Some(p) = lib_content.get(&file_name) {
                    let source_attr = fs::metadata(&source_media);
                    let asset_attr = fs::metadata(p);
                    if !source_attr.is_ok() || !asset_attr.is_ok() {
                        return None;
                    }
                    let source_attr = source_attr.unwrap();
                    let asset_attr = asset_attr.unwrap();
                    if source_attr.len() == asset_attr.len() {
                        return Some(source_media.clone());
                    }
                }
            }
            None
        })
        .collect();
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let library = Path::new(&args.arg_imovielibrary);
    let assets = list_media_assets(library);

    let source = Path::new(&args.arg_source);
    let source_assets = list_source_assets(source);

    let dupes = intersect(assets, source_assets);

    for dupe in dupes {
        println!("{:?}", dupe);
    }
}
