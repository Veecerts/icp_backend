use std::io;

use async_graphql::{Error, Result};
use pinata_sdk::{PinByFile, PinataApi, PinnedObject};
use tempfile::NamedTempFile;

use crate::config::settings::ENV;

pub struct Pinata;

impl Pinata {
    fn init() -> Result<PinataApi> {
        let env = ENV::init();
        let pinata_api_key = env.pinata_api_key;
        let pinata_api_secret = env.pinata_api_secret;
        let api = PinataApi::new(pinata_api_key, pinata_api_secret)?;
        return Ok(api);
    }

    pub async fn pin_file(file: std::fs::File) -> Result<PinnedObject> {
        let api = Pinata::init()?;
        let mut temp_file = NamedTempFile::new()?;
        io::copy(&mut &file, &mut temp_file)?;
        let temp_file_path = temp_file.path().to_str();
        if let Some(file_path) = temp_file_path {
            let response = api.pin_file(PinByFile::new(file_path)).await?;
            Ok(response)
        } else {
            Err(Error::new("Failed to pin file"))
        }
    }

    pub async fn unpin_file(hash: &str) -> Result<()> {
        let api = Pinata::init()?;
        api.unpin(hash).await?;
        Ok(())
    }

    pub fn build_url(hash: String) -> String {
        let pinata_ipfs_gateway = ENV::init().pinata_ipfs_gateway;
        format!("https://{}/ips/{}", pinata_ipfs_gateway, hash)
    }
}
