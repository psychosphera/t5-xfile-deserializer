use std::{ffi::OsString, path::Path, str::FromStr};

use t5_xfile_deserializer::{T5XFileDeserializerBuilder, XFilePlatform};
fn main() {
    let filename = std::env::args_os()
        .nth(1)
        .unwrap_or(OsString::from_str("en_frontend.ff").unwrap());
    let cached_filename = Path::new(&filename).with_extension("cache");
    let cache_exists = Path::new(&filename).with_extension("cache").exists();

    let mut file = if cache_exists {
        std::fs::File::open(&cached_filename).unwrap()
    } else {
        std::fs::File::open(&filename).unwrap()
    };

    let de = if cache_exists {
        println!("Found inflated cache file, reading...");
        T5XFileDeserializerBuilder::from_cache_file(&mut file, XFilePlatform::Windows)
    } else {
        T5XFileDeserializerBuilder::from_file(&mut file, XFilePlatform::Windows)
    }
    .with_silent(false);

    #[cfg(feature = "d3d9")]
    let de = de.with_d3d9(None);

    let de = de.build().unwrap().inflate().unwrap();

    let de = if !cache_exists {
        de.cache(cached_filename).unwrap().0
    } else {
        de.no_cache().unwrap()
    };

    let assets = de.deserialize_remaining().unwrap();
    for (i, asset) in assets.into_iter().enumerate() {
        println!("Found asset '{}' ({})", asset.name().unwrap_or_default(), i);
    }
    //dbg!(assets);
}
