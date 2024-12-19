use async_graphql::*;
use candid::{Decode, Encode, Principal};
use chrono::Utc;
use ic_agent::Agent;
use log::info;
use serde::Serialize;

use crate::config::settings::ENV;

#[derive(Serialize, candid::Deserialize)]
struct Asset {
    uuid: String,
    ipfs_hash: String,
    date_added: String,
}

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct NFTDetails {
    pub id: u64,
    pub owner: Principal,
    pub metadata: String,
    pub collection_id: u64,
}

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct MintNFTSuccess {
    pub txn_id: u128,
    pub nft: NFTDetails,
}

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub enum NFTError {
    TokenNotFound,
    CollectionNotFound,
    InvalidTokenID,
    Unauthorized,
}

impl std::fmt::Display for NFTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NFTError::TokenNotFound => write!(f, "Token not found"),
            NFTError::CollectionNotFound => write!(f, "Collection not found"),
            NFTError::InvalidTokenID => write!(f, "Invalid Token ID"),
            NFTError::Unauthorized => write!(f, "Unauthorized access"),
        }
    }
}

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub enum MintNFTResult {
    Ok(MintNFTSuccess),
    Err(NFTError),
}

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub enum BurnNFTResult {
    Ok(u128),
    Err(NFTError),
}

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub struct NFTCollectionDetails {
    pub id: u64,
    pub owner: Principal,
    pub logo: Option<String>,
    pub name: String,
    pub description: String,
    pub symbol: String,
}

// #[derive(candid::CandidType, candid::Deserialize, Debug)]
// pub struct CreateNFTSuccess {
//     pub txn_id: u128,
//     pub nft: NFTCollectionDetails,
// }

#[derive(candid::CandidType, candid::Deserialize, Debug)]
pub enum CreateNFTResult {
    Ok((u128, NFTCollectionDetails)),
    Err(NFTError),
}

pub struct Contract;

impl Contract {
    fn init() -> Result<(Principal, Agent)> {
        let env = ENV::init();
        let icp_agent_endpoint = env.icp_agent_endpoint;
        let canister_principal_id = env.canister_principal_id;
        let canister_id = Principal::from_text(canister_principal_id.as_str())?;
        let agent = Agent::builder().with_url(icp_agent_endpoint).build()?;
        Ok((canister_id, agent))
    }

    pub async fn mint_nft(
        collection_id: i64,
        uuid: &String,
        ipfs_hash: &String,
    ) -> Result<MintNFTResult> {
        let (canister_id, agent) = Contract::init()?;
        let method_name = "mint_nft";
        let contract_asset = Asset {
            uuid: uuid.to_string(),
            ipfs_hash: ipfs_hash.clone(),
            date_added: Utc::now().to_string(),
        };
        let metadata = serde_json::to_string(&contract_asset)?;

        let args = Encode!(&collection_id, &metadata)?;

        let response = agent
            .update(&canister_id, method_name)
            .with_arg(args)
            .call_and_wait()
            .await?;

        let result = Decode!(&response, MintNFTResult)?;
        Ok(result)
    }

    pub async fn burn_nft(token_id: String) -> Result<BurnNFTResult> {
        let (canister_id, agent) = Contract::init()?;
        let method_name = "burn_nft";

        let args = Encode!(&token_id)?;
        let response = agent
            .update(&canister_id, method_name)
            .with_arg(args)
            .call_and_wait()
            .await?;

        let result = Decode!(&response, BurnNFTResult)?;
        return Ok(result);
    }

    pub async fn create_nft(
        name: &String,
        symbol: &String,
        description: &String,
        logo: &Option<String>,
    ) -> Result<CreateNFTResult> {
        let (canister_id, agent) = Contract::init()?;
        let method_name = "create_nft";

        let args = Encode!(name, symbol, description, logo)?;
        let response = agent
            .update(&canister_id, method_name)
            .with_arg(args)
            .call_and_wait()
            .await?;

        let result = Decode!(&response, CreateNFTResult)?;

        info!("DECODED {:?}", &result);
        Ok(result)
    }
}
