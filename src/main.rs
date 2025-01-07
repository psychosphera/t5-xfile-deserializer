use std::path::Path;

use t5_xfile_deserializer::{T5XFileDeserializerBuilder, XFilePlatform};

use clap::{arg, command};

fn main() {
    let matches = command!()
        .arg(arg!([FILENAME] "Filename to use (should have .ff or .cache extension)"))
        .arg(
            arg!(
                -p --platform <PLATFORM> 
                "Specifies which platform the Fastfile is expected to be for. Should be one of:\n\
                 \twindows\n\
                 \tmacos\n\
                 \txbox360\n\
                 \tps3\n\
                 \twii"
            )
        )
        .arg(
            arg!(
                -a --allow_unsupported_platforms 
                "Permits the deserializer to operate on platforms that may not be fully supported. \
                 Will probably cause problems."
            ).required(false)
        )
        .get_matches();

    let Some(filename) = matches.get_one::<String>("FILENAME") else {
        println!("must specify a file to operate on (should have .ff or .cache extension)");
        return;
    };

    let platform = if let Some(p) = matches.get_one::<String>("platform") {
        let p = p.as_str();
        match p {
            "windows" => XFilePlatform::Windows,
            "macos" => XFilePlatform::macOS,
            "xbox360" => XFilePlatform::Xbox360,
            "ps3" => XFilePlatform::PS3,
            "wii" => XFilePlatform::Wii,
            _ => {
                println!("invalid platform (see --help for a list of valid platforms)");
                return;
            }
        }
    } else {
        println!("must specify the expected platform for the Fastfile (-p/--platform, see --help for a list of valid platforms)");
        return;
    };

    let cached_filename = Path::new(&filename).with_extension("cache");
    let cache_exists = Path::new(&filename).with_extension("cache").exists();

    let mut file = if cache_exists {
        std::fs::File::open(&cached_filename).unwrap()
    } else {
        std::fs::File::open(&filename).unwrap()
    };

    let allow_unsupported_platforms = matches.get_one::<bool>("allow_unsupported_platforms").is_some();

    let de = if cache_exists {
        T5XFileDeserializerBuilder::from_cache_file(&mut file, platform, allow_unsupported_platforms)
    } else {
        T5XFileDeserializerBuilder::from_file(&mut file, platform, allow_unsupported_platforms)
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
