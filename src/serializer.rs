use core::marker::PhantomData;

use serde::Serialize;

use std::{
    collections::HashSet,
    io::{Cursor, Seek},
};

use crate::{file_line_col, BincodeOptions, StreamLen};

use t5_xfile_defs::{
    xasset::{XAsset, XAssetList},
    Error, ErrorKind, Result, T5XFileSerialize, XFile, XFileHeader, XFilePlatform,
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

    pub fn build(self) -> Result<T5XFileSerializer<T5XFileSerializerSerialize>> {
        T5XFileSerializer::new(self.silent, self.platform)
    }
}

/// Trait to seal [`T5XFileSerializer`]'s typestates.
pub(crate) trait T5XFileSerializerTypestate {}

pub enum T5XFileSerializerSerialize {}
pub enum T5XFileSerializerInflated {}
pub enum T5XFileSerializerDeflated {}

impl T5XFileSerializerTypestate for T5XFileSerializerSerialize {}
impl T5XFileSerializerTypestate for T5XFileSerializerInflated {}
impl T5XFileSerializerTypestate for T5XFileSerializerDeflated {}

#[allow(private_bounds, private_interfaces)]
pub struct T5XFileSerializer<T: T5XFileSerializerTypestate = T5XFileSerializerSerialize> {
    _silent: bool,
    xfile: XFile,
    xasset_list: XAssetList,
    script_strings: HashSet<String>,
    asset_bytes: Cursor<Vec<u8>>,
    serialized_assets: usize,
    opts: BincodeOptions,
    platform: XFilePlatform,
    _p: PhantomData<T>,
}

impl<'a> T5XFileSerializer<T5XFileSerializerSerialize> {
    pub fn new(silent: bool, platform: XFilePlatform) -> Result<Self> {
        Ok(Self {
            _silent: silent,
            xfile: XFile::default(),
            xasset_list: XAssetList::default(),
            script_strings: HashSet::new(),
            asset_bytes: Cursor::new(Vec::new()),
            serialized_assets: 0,
            opts: BincodeOptions::from_platform(platform),
            platform,
            _p: PhantomData,
        })
    }

    pub fn serialize<const MAX_LOCAL_CLIENTS: usize>(
        &mut self,
        assets: impl Iterator<Item = XAsset>,
    ) -> Result<()> {
        for asset in assets {
            self.store_into_xfile(asset.clone())?;
            self.xasset_list.assets.push(asset);
            self.serialized_assets += 1;
        }

        Ok(())
    }

    pub fn deflate(mut self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        let header = XFileHeader::new(self.platform);

        self.opts.serialize_into(&mut bytes, header).map_err(|e| {
            Error::new(
                file_line_col!(),
                self.stream_pos().unwrap() as _,
                ErrorKind::Bincode(e),
            )
        })?;

        let mut blob = Cursor::new(Vec::new());

        self.opts
            .serialize_into(&mut blob, self.xfile)
            .map_err(|e| {
                Error::new(
                    file_line_col!(),
                    self.stream_pos().unwrap() as _,
                    ErrorKind::Bincode(e),
                )
            })?;

        // TODO: serialize XAssets

        let deflated_blob = deflate::deflate_bytes_zlib(&blob.into_inner());

        bytes.extend_from_slice(&deflated_blob);

        Ok(bytes)
    }
}

impl T5XFileSerialize for T5XFileSerializer {
    fn stream_pos(&mut self) -> Result<u64> {
        self.asset_bytes
            .stream_position()
            .map_err(|e| Error::new(file_line_col!(), 0, ErrorKind::Io(e)))
    }

    fn stream_len(&mut self) -> Result<u64> {
        StreamLen::stream_len(&mut self.asset_bytes)
    }

    fn store_into_xfile<T: Serialize>(&mut self, t: T) -> Result<()> {
        self.opts
            .serialize_into(&mut self.asset_bytes, t)
            .map_err(|e| {
                Error::new(
                    file_line_col!(),
                    self.stream_pos().unwrap() as _,
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
