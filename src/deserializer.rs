use core::marker::PhantomData;

use alloc::collections::VecDeque;
use serde::de::DeserializeOwned;

use std::{
    io::{Cursor, Read, Seek, Write},
    path::Path,
};

use crate::{BincodeOptions, Error, ErrorKind, Result, StreamLen, file_line_col, size_of};

use t5_xfile_defs::{
    FatPointer, ScriptString, T5XFileDeserialize, XFile, XFileDeserializeInto, XFileHeader,
    XFilePlatform, XFileVersion,
    xasset::{XAsset, XAssetListRaw, XAssetRaw},
};

pub enum InflateSuccess {
    NewlyInflated,
    AlreadyInflated,
}

pub enum CacheSuccess {
    CacheCreated,
    CacheOverwritten,
}

#[cfg(feature = "d3d9")]
pub struct D3D9State<'a> {
    pub(crate) device: &'a mut IDirect3DDevice9,
}

#[cfg(not(feature = "d3d9"))]
pub(crate) struct D3D9State<'a>(PhantomData<&'a ()>);

/// Trait to seal [`T5XFileDeserializer`]'s typestates.
pub(crate) trait T5XFileDeserializerTypestate {}

pub enum T5XFileDeserializerDeflated {}
pub enum T5XFileDeserializerInflated {}
pub enum T5XFileDeserializerDeserialize {}

impl T5XFileDeserializerTypestate for T5XFileDeserializerDeflated {}
impl T5XFileDeserializerTypestate for T5XFileDeserializerInflated {}
impl T5XFileDeserializerTypestate for T5XFileDeserializerDeserialize {}

#[allow(private_bounds, private_interfaces)]
pub struct T5XFileDeserializer<'a, T: T5XFileDeserializerTypestate = T5XFileDeserializerDeserialize>
{
    silent: bool,
    xfile: XFile,
    script_strings: Vec<String>,
    file: Option<&'a mut std::fs::File>,
    cache_file: Option<&'a mut std::fs::File>,
    reader: Option<Cursor<Vec<u8>>>,
    xasset_list: XAssetListRaw<'a>,
    xassets_raw: VecDeque<XAssetRaw<'a>>,
    deserialized_assets: usize,
    non_null_assets: usize,
    opts: BincodeOptions,
    platform: XFilePlatform,
    d3d9_state: Option<D3D9State<'a>>,
    _p: PhantomData<T>,
}

pub struct T5XFileDeserializerBuilder<'a> {
    file: Option<&'a mut std::fs::File>,
    cache_file: Option<&'a mut std::fs::File>,
    silent: bool,
    platform: XFilePlatform,
    allow_unsupported_platforms: bool,
    d3d9_state: Option<D3D9State<'a>>,
}

impl<'a> T5XFileDeserializerBuilder<'a> {
    pub fn from_file(
        file: &'a mut std::fs::File,
        platform: XFilePlatform,
        allow_unsupported_platforms: bool,
    ) -> Self {
        Self {
            file: Some(file),
            cache_file: None,
            platform,
            silent: false,
            allow_unsupported_platforms,
            d3d9_state: None,
        }
    }

    pub fn from_cache_file(
        cache_file: &'a mut std::fs::File,
        platform: XFilePlatform,
        allow_unsupported_platforms: bool,
    ) -> Self {
        Self {
            file: None,
            cache_file: Some(cache_file),
            platform,
            silent: false,
            allow_unsupported_platforms,
            d3d9_state: None,
        }
    }

    pub fn with_silent(mut self, silent: bool) -> Self {
        self.silent = silent;
        self
    }

    pub fn with_allow_unsupported_platforms(mut self, allow_unsupported_platforms: bool) -> Self {
        self.allow_unsupported_platforms = allow_unsupported_platforms;
        self
    }

    #[cfg(feature = "d3d9")]
    pub fn with_d3d9(mut self, d3d9_state: Option<D3D9State<'a>>) -> Self {
        self.d3d9_state = d3d9_state;
        self
    }

    pub fn build(mut self) -> Result<T5XFileDeserializer<'a, T5XFileDeserializerDeflated>> {
        if self.file.is_some() {
            T5XFileDeserializer::from_file(
                self.file.take().unwrap(),
                self.silent,
                self.allow_unsupported_platforms,
                self.platform,
                self.d3d9_state,
            )
        } else if self.cache_file.is_some() {
            T5XFileDeserializer::from_cache_file(
                self.cache_file.take().unwrap(),
                self.silent,
                self.allow_unsupported_platforms,
                self.platform,
                self.d3d9_state,
            )
        } else {
            unreachable!()
        }
    }
}

impl<'a> T5XFileDeserializer<'a, T5XFileDeserializerDeflated> {
    fn from_file(
        file: &'a mut std::fs::File,
        silent: bool,
        allow_unsupported_platforms: bool,
        platform: XFilePlatform,
        d3d9_state: Option<D3D9State<'a>>,
    ) -> Result<Self> {
        if platform == XFilePlatform::Wii {
            if !silent {
                println!("Error: Wii Fastfiles are unimplemented.");
            }

            return Err(Error::new_with_offset(
                file_line_col!(),
                0,
                ErrorKind::UnimplementedPlatform(platform),
            ));
        }

        if platform == XFilePlatform::Xbox360 || platform == XFilePlatform::PS3 {
            if allow_unsupported_platforms && !silent {
                println!(
                    "Warning: {platform} Fastfiles might (and probably do) have differences \
                     from Windows Fastfiles that aren't accounted for in this \
                     library. Expect problems."
                );
            } else {
                if !silent {
                    println!(
                        "Error: {platform} Fastfiles might (and probably do) have differences \
                         from Windows Fastfiles that aren't accounted for in this \
                         library, and as such, they are unsupported."
                    );
                }
                return Err(Error::new_with_offset(
                    file_line_col!(),
                    0,
                    ErrorKind::UnsupportedPlatform(platform),
                ));
            }
        }

        if !silent && platform == XFilePlatform::macOS {
            if allow_unsupported_platforms {
                println!(
                    "Warning: macOS Fastfiles are *presumably* identical to \
                     Windows Fastfiles (being an Aspyr port and all), but the \
                     author of this library hasn't yet verified that to be true. \
                     Problems may arise."
                );
            } else {
                println!(
                    "Error: macOS Fastfiles are *presumably* identical to \
                     Windows Fastfiles (being an Aspyr port and all), but the \
                     author of this library hasn't yet verified that to be true, \
                     and as such, they are unsupported."
                );
                return Err(Error::new_with_offset(
                    file_line_col!(),
                    0,
                    ErrorKind::UnsupportedPlatform(platform),
                ));
            }
        }

        if !silent {
            println!("Found file, reading header...");
        }

        let opts = BincodeOptions::from_platform(platform);

        let header = opts
            .deserialize_from::<XFileHeader>(&mut *file)
            .map_err(|e| Error::new_with_offset(file_line_col!(), 0, ErrorKind::Bincode(e)))?;

        // dbg!(&header);

        if !header.magic_is_valid() {
            if !silent {
                println!("Fastfile header magic invalid: valid values are IWffu100 and IWff0100");
            }
            return Err(Error::new_with_offset(
                file_line_col!(),
                0,
                ErrorKind::BadHeaderMagic(header.magic_string()),
            ));
        }

        if XFileVersion::is_other_endian(header.version) {
            if !silent {
                println!(
                    "Fastfile header is valid, but it has the wrong endianness \
                     for {platform} (probably for a different platform)."
                );
            }
            return Err(Error::new_with_offset(
                file_line_col!(),
                0,
                ErrorKind::WrongEndiannessForPlatform(platform),
            ));
        }

        if !XFileVersion::is_valid(header.version, platform) {
            if !silent {
                println!(
                    "Fastfile is wrong version (version={:#010X}, expected {:#010X})",
                    header.version,
                    XFileVersion::from_platform(platform).as_u32()
                );
            }

            return Err(Error::new_with_offset(
                file_line_col!(),
                0,
                ErrorKind::WrongVersion(header.version),
            ));
        }

        if !silent {
            println!("Header verified, reading playload...");
        }

        let de = Self {
            silent,
            xfile: XFile::default(),
            script_strings: Vec::default(),
            file: Some(file),
            cache_file: None,
            reader: None,
            xasset_list: XAssetListRaw::default(),
            xassets_raw: VecDeque::new(),
            deserialized_assets: 0,
            non_null_assets: 0,
            opts,
            platform,
            d3d9_state,
            _p: PhantomData,
        };

        Ok(de)
    }

    fn from_cache_file(
        file: &'a mut std::fs::File,
        silent: bool,
        allow_unsupported_platforms: bool,
        platform: XFilePlatform,
        d3d9_state: Option<D3D9State<'a>>,
    ) -> Result<Self> {
        if platform == XFilePlatform::Wii {
            if !silent {
                println!("Error: Wii Fastfiles are unimplemented.");
            }

            return Err(Error::new_with_offset(
                file_line_col!(),
                0,
                ErrorKind::UnimplementedPlatform(platform),
            ));
        }

        if platform == XFilePlatform::Xbox360 || platform == XFilePlatform::PS3 {
            if allow_unsupported_platforms && !silent {
                println!(
                    "Warning: {platform} Fastfiles might (and probably do) have differences \
                     from Windows Fastfiles that aren't accounted for in this \
                     library. Expect problems."
                );
            } else {
                if !silent {
                    println!(
                        "Error: {platform} Fastfiles might (and probably do) have differences \
                         from Windows Fastfiles that aren't accounted for in this \
                         library, and as such, they are unsupported."
                    );
                }
                return Err(Error::new_with_offset(
                    file_line_col!(),
                    0,
                    ErrorKind::UnsupportedPlatform(platform),
                ));
            }
        }

        if !silent && platform == XFilePlatform::macOS {
            if allow_unsupported_platforms {
                println!(
                    "Warning: macOS Fastfiles are *presumably* identical to \
                     Windows Fastfiles (being an Aspyr port and all), but the \
                     author of this library hasn't yet verified that to be true. \
                     Problems may arise."
                );
            } else {
                println!(
                    "Error: macOS Fastfiles are *presumably* identical to \
                     Windows Fastfiles (being an Aspyr port and all), but the \
                     author of this library hasn't yet verified that to be true, \
                     and as such, they are unsupported."
                );
                return Err(Error::new_with_offset(
                    file_line_col!(),
                    0,
                    ErrorKind::UnsupportedPlatform(platform),
                ));
            }
        }

        if !silent {
            println!("Found inflated cache file, reading...");
        }

        Ok(T5XFileDeserializer::<'a, T5XFileDeserializerDeflated> {
            silent,
            xfile: XFile::default(),
            script_strings: Vec::default(),
            file: None,
            cache_file: Some(file),
            reader: None,
            xasset_list: XAssetListRaw::default(),
            xassets_raw: VecDeque::new(),
            deserialized_assets: 0,
            non_null_assets: 0,
            opts: BincodeOptions::from_platform(platform),
            platform,
            d3d9_state,
            _p: PhantomData,
        })
    }

    pub fn inflate(mut self) -> Result<T5XFileDeserializer<'a, T5XFileDeserializerInflated>> {
        assert!(self.reader.is_none());

        let reader = if let Some(f) = self.cache_file.take() {
            let mut decompressed_payload = Vec::new();
            f.read_to_end(&mut decompressed_payload)
                .map_err(|e| Error::new_with_offset(file_line_col!(), 0, ErrorKind::Io(e)))?;
            Cursor::new(decompressed_payload)
        } else if let Some(f) = self.file.take() {
            let mut compressed_payload = Vec::new();
            f.seek(std::io::SeekFrom::Start(size_of!(XFileHeader) as _))
                .map_err(|e| Error::new_with_offset(file_line_col!(), 0, ErrorKind::Io(e)))?;
            // dbg!(f.stream_position().map_err(|e| Error::new_with_offset(
            //     file_line_col!(),
            //     0,
            //     ErrorKind::Io(e)
            // ))?);
            let bytes_read = f
                .read_to_end(&mut compressed_payload)
                .map_err(|e| Error::new_with_offset(file_line_col!(), 0, ErrorKind::Io(e)))?;
            if !self.silent {
                println!("Payload read, inflating... (this may take a while)");
            }
            let decompressed_payload = inflate::inflate_bytes_zlib(&compressed_payload)
                .map_err(|e| Error::new_with_offset(file_line_col!(), 0, ErrorKind::Inflate(e)))?;
            if !self.silent {
                println!(
                    "Payload inflated, compressed size: {} bytes, decompressed size: {} bytes",
                    bytes_read,
                    decompressed_payload.len()
                );
            }
            Cursor::new(decompressed_payload)
        } else {
            unreachable!() // safe since the constructors had to populate at least self.cache_file
        };

        self.reader = Some(reader);

        let xasset_list = {
            let mut file = self.reader.as_mut().unwrap();
            let xfile = self
                .opts
                .deserialize_from::<XFile>(&mut file)
                .map_err(|e| Error::new_with_offset(file_line_col!(), 0, ErrorKind::Bincode(e)))?;

            dbg!(xfile);
            //dbg!(StreamLen::stream_len(&mut file)?);
            self.xfile = xfile;

            // dbg!(file.stream_position().map_err(|e| Error::new(
            //     file_line_col!(),
            //     0,
            //     ErrorKind::Io(e)
            // ))?);
            let xasset_list = self
                .opts
                .deserialize_from::<XAssetListRaw>(&mut file)
                .map_err(|e| Error::new_with_offset(file_line_col!(), 0, ErrorKind::Bincode(e)))?;
            //dbg!(&xasset_list);
            // dbg!(file.stream_position().map_err(|e| Error::new(
            //     file_line_col!(),
            //     0,
            //     ErrorKind::Io(e)
            // ))?);
            xasset_list
        };

        if !self.silent {
            println!("Fastfile contains {} assets.", xasset_list.assets.size());
        }

        let de = T5XFileDeserializer::<T5XFileDeserializerInflated> {
            silent: self.silent,
            xfile: self.xfile,
            script_strings: Vec::new(),
            file: self.file,
            cache_file: self.cache_file,
            reader: self.reader,
            xasset_list,
            xassets_raw: VecDeque::new(),
            deserialized_assets: self.deserialized_assets,
            non_null_assets: self.non_null_assets,
            opts: self.opts,
            platform: self.platform,
            d3d9_state: self.d3d9_state,
            _p: PhantomData,
        };

        Ok(de)
    }
}

impl<'a> T5XFileDeserializer<'a, T5XFileDeserializerInflated> {
    pub fn cache(
        mut self,
        path: impl AsRef<Path>,
    ) -> Result<(
        T5XFileDeserializer<'a, T5XFileDeserializerDeserialize>,
        CacheSuccess,
    )> {
        if !self.silent {
            println!("Caching decompressed payload to disk...");
        }

        let cache_exists = path.as_ref().exists();

        let mut f = std::fs::File::create(path)
            .map_err(|e| Error::new_with_offset(file_line_col!(), 0, ErrorKind::Io(e)))?;
        let pos = self.reader.as_ref().unwrap().position();
        let v = self.reader.take().unwrap().into_inner();
        f.write_all(&v)
            .map_err(|e| Error::new_with_offset(file_line_col!(), 0, ErrorKind::Io(e)))?;
        self.reader = Some(Cursor::new(v));
        self.reader.as_mut().unwrap().set_position(pos);

        if !self.silent {
            println!("Decompressed payload cached.");
        }

        let mut de = T5XFileDeserializer::<'a, T5XFileDeserializerDeserialize> {
            silent: self.silent,
            xfile: self.xfile,
            script_strings: Vec::new(),
            file: self.file,
            cache_file: self.cache_file,
            reader: self.reader,
            xasset_list: self.xasset_list,
            xassets_raw: self.xassets_raw,
            deserialized_assets: self.deserialized_assets,
            non_null_assets: self.non_null_assets,
            opts: self.opts,
            platform: self.platform,
            d3d9_state: self.d3d9_state,
            _p: PhantomData,
        };

        de.get_script_strings_and_assets()?;

        if cache_exists {
            Ok((de, CacheSuccess::CacheOverwritten))
        } else {
            Ok((de, CacheSuccess::CacheCreated))
        }
    }

    pub fn no_cache(self) -> Result<T5XFileDeserializer<'a, T5XFileDeserializerDeserialize>> {
        let mut de = T5XFileDeserializer::<'a, T5XFileDeserializerDeserialize> {
            silent: self.silent,
            xfile: self.xfile,
            script_strings: Vec::new(),
            file: self.file,
            cache_file: self.cache_file,
            reader: self.reader,
            xasset_list: self.xasset_list,
            xassets_raw: self.xassets_raw,
            deserialized_assets: self.deserialized_assets,
            non_null_assets: self.non_null_assets,
            opts: self.opts,
            platform: self.platform,
            d3d9_state: self.d3d9_state,
            _p: PhantomData,
        };

        de.get_script_strings_and_assets()?;

        Ok(de)
    }
}

impl<'a> T5XFileDeserializer<'a, T5XFileDeserializerDeserialize> {
    pub fn deserialize_next(&mut self) -> Result<Option<XAsset>> {
        let Some(asset) = self.xassets_raw.pop_front() else {
            return Ok(None);
        };

        let asset = XAsset::try_get(self, asset, self.platform);
        //dbg!(&asset);
        if let Ok(ref a) = asset {
            self.deserialized_assets += 1;
            if a.is_some() {
                self.non_null_assets += 1;
            }

            if !self.silent {
                println!(
                    "Successfully deserialized {} asset{} ({} non-null).",
                    self.deserialized_assets,
                    if self.deserialized_assets > 1 {
                        "s"
                    } else {
                        ""
                    },
                    self.non_null_assets,
                );
            }
        }

        asset.map(Some)
    }

    pub fn deserialize_remaining(mut self) -> Result<Vec<XAsset>> {
        let mut deserialized_assets = Vec::new();

        while let Some(asset) = self.deserialize_next()? {
            deserialized_assets.push(asset);
        }

        Ok(deserialized_assets)
    }

    fn get_script_strings_and_assets(&mut self) -> Result<()> {
        let xasset_list = self.xasset_list;

        self.script_strings = xasset_list
            .strings
            .to_vec(self)?
            .into_iter()
            .map(|s| s.xfile_deserialize_into(self, ()).map(|s| s.0))
            .collect::<Result<Vec<_>>>()?;
        //dbg!(&strings);

        let assets = xasset_list.assets.to_vec(self)?;
        self.xassets_raw = VecDeque::from_iter(assets);

        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) const fn create_d3d9(&self) -> bool {
        self.d3d9_state.is_some()
    }

    #[allow(dead_code)]
    pub(crate) fn d3d9_state(&mut self) -> Option<&mut D3D9State<'a>> {
        self.d3d9_state.as_mut()
    }
}

impl<'a> T5XFileDeserialize for T5XFileDeserializer<'a> {
    fn stream_pos(&mut self) -> Result<u64> {
        self.reader
            .as_mut()
            .unwrap()
            .stream_position()
            .map_err(|e| Error::new_with_offset(file_line_col!(), 0, ErrorKind::Io(e)))
    }

    fn stream_len(&mut self) -> Result<u64> {
        StreamLen::stream_len(self.reader.as_mut().unwrap())
    }

    fn load_from_xfile<T: DeserializeOwned>(&mut self) -> Result<T> {
        // FIXME: unwrap
        self.opts
            .deserialize_from(self.reader.as_mut().unwrap())
            .map_err(|e| {
                Error::new_with_offset(
                    file_line_col!(),
                    self.stream_pos().unwrap() as _,
                    ErrorKind::Bincode(e),
                )
            })
    }

    fn get_script_string(&self, string: ScriptString) -> Result<Option<&str>> {
        Ok(self.script_strings.get(string.as_u16() as usize).map(|s| &**s))
    }
}
