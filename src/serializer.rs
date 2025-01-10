use serde::Serialize;

use std::{
    collections::HashSet,
    io::{Cursor, Seek, Write},
};

use crate::{file_line_col, BincodeOptions};

use t5_xfile_defs::{
    xasset::{XAsset, XAssetListRaw}, Error, ErrorKind, FatPointerCountFirstU32, Ptr32, Result, T5XFileSerialize, XFile, XFileHeader, XFilePlatform, XFileSerialize
};

pub struct T5XFileSerializerBuilder {
    silent: bool,
    platform: XFilePlatform,
}

impl T5XFileSerializerBuilder {
    pub fn new(platform: XFilePlatform) -> Self {
        Self {
            platform,
            silent: false,
        }
    }

    pub fn with_silent(mut self, silent: bool) -> Self {
        self.silent = silent;
        self
    }

    pub fn build(self) -> Result<T5XFileSerializer> {
        T5XFileSerializer::new(self.silent, self.platform)
    }
}

#[allow(private_bounds, private_interfaces)]
pub struct T5XFileSerializer {
    _silent: bool,
    xfile: XFile,
    script_strings: HashSet<String>,
    asset_bytes: Option<Cursor<Vec<u8>>>,
    serialized_assets: usize,
    opts: BincodeOptions,
    platform: XFilePlatform,
}

impl<'a> T5XFileSerializer {
    pub fn new(silent: bool, platform: XFilePlatform) -> Result<Self> {
        Ok(Self {
            _silent: silent,
            xfile: XFile::default(),
            script_strings: HashSet::new(),
            asset_bytes: None,
            serialized_assets: 0,
            opts: BincodeOptions::from_platform(platform),
            platform,
        })
    }

    pub fn serialize_assets<const MAX_LOCAL_CLIENTS: usize>(
        &mut self,
        assets: impl Iterator<Item = XAsset>,
    ) -> Result<()> {
        for asset in assets {
            asset.xfile_serialize(self, ());
            self.serialized_assets += 1;
        }

        Ok(())
    }

    fn serialize<T: Serialize>(&mut self, mut writer: impl Write + Seek, t: T) -> Result<()> {
        self.opts.serialize_into(&mut writer, t).map_err(|e| {
            Error::new(
                file_line_col!(),
                writer.stream_position().unwrap() as _,
                ErrorKind::Bincode(e),
            )
        })
    }

    pub fn deflate(mut self) -> Result<Vec<u8>> {
        let mut bytes = Cursor::new(Vec::new());
        let header = XFileHeader::new(self.platform);

        self.serialize(&mut bytes, header)?;

        let mut blob = Cursor::new(Vec::new());

        self.serialize(&mut blob, self.xfile)?;

        // TODO: serialize XAssets
        let xasset_list = XAssetListRaw { 
            strings: FatPointerCountFirstU32 { size: self.script_strings.len() as _, p: Ptr32::from_u32(0xFFFFFFFF) },
            assets: FatPointerCountFirstU32 { size: self.serialized_assets as _, p: Ptr32::from_u32(0xFFFFFFFF) },
        };

        self.serialize(&mut blob, xasset_list)?;

        let mut script_string_bytes = Vec::new();
        for string in self.script_strings.iter() {
            for c in string.chars() {
                script_string_bytes.push(c as u8);
            }
            script_string_bytes.push(b'\0');
        }

        self.serialize(&mut blob, script_string_bytes)?;
        let asset_bytes = self.asset_bytes.take().unwrap_or_default().into_inner();
        self.serialize(&mut blob, asset_bytes)?;

        let deflated_blob = deflate::deflate_bytes_zlib(&blob.into_inner());

        let mut bytes = bytes.into_inner();
        bytes.extend_from_slice(&deflated_blob);

        Ok(bytes)
    }
}

impl T5XFileSerialize for T5XFileSerializer {
    fn store_into_xfile<T: Serialize>(&mut self, t: T) -> Result<()> {
        self.opts
            .serialize_into(self.asset_bytes.get_or_insert(Cursor::new(Vec::new())), t)
            .map_err(|e| {
                Error::new(
                    file_line_col!(),
                    self.asset_bytes.as_ref().unwrap().position() as _,
                    ErrorKind::Bincode(e),
                )
            })
    }

    fn get_or_insert_script_string(&mut self, string: String) -> Result<Option<String>> {
        if self.script_strings.len() >= u16::MAX as usize {
            Ok(None)
        } else {
            self.script_strings.insert(string.clone());
            Ok(Some(self.script_strings.get(&string).unwrap().clone()))
        }
    }
}
