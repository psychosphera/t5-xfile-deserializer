#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![feature(seek_stream_len)]

use std::{ffi::OsString, path::Path, str::FromStr};

use t5_xfile_deserializer::{T5XFileDeserializer, XFilePlatform};
fn main() {
    let filename = std::env::args_os()
        .nth(1)
        .unwrap_or(OsString::from_str("cuba.ff").unwrap());
    let cached_filename = Path::new(&filename).with_extension("cache");
    let cache_exists = Path::new(&filename).with_extension("cache").exists();

    let mut file = if cache_exists {
        std::fs::File::open(&cached_filename).unwrap()
    } else {
        std::fs::File::open(&filename).unwrap()
    };

    let mut de = if cache_exists {
        println!("Found inflated cache file, reading...");
        T5XFileDeserializer::from_cache_file(&mut file, false, XFilePlatform::Windows).unwrap()
    } else {
        T5XFileDeserializer::from_file(&mut file, false, false, XFilePlatform::Windows).unwrap()
    };

    de.inflate().unwrap();
    if !cache_exists {
        de.cache(cached_filename).unwrap();
    }
    let assets = de.deserialize_remaining().unwrap();
    for (i, asset) in assets.iter().enumerate() {
        println!("Found asset '{}' ({})", asset.name().unwrap_or_default(), i);
    }
    //dbg!(assets);
}
