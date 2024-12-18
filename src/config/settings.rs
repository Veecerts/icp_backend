use std::env;

pub struct ENV {
    pub port: u16,
    pub addrs: String,
    pub db_url: String,
    pub allowed_origns: String,
    pub secret_key: String,
    pub icp_agent_endpoint: String,
    pub canister_principal_id: String,
    pub pinata_api_key: String,
    pub pinata_api_secret: String,
    pub pinata_jwt: String,
    pub pinata_ipfs_gateway: String,
}

impl ENV {
    pub fn init() -> ENV {
        let port = env::var("PORT")
            .expect("PORT environment variable must be set")
            .parse::<u16>()
            .expect("PORT should be a valid number");
        let addrs =
            env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS environment variable must be set");
        let db_url =
            env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");
        let allowed_origns =
            env::var("ALLOWED_ORIGINS").expect("ALLOWED_ORIGINS environment variable must be set");

        let secret_key =
            env::var("SECRET_KEY").expect("SECRET_KEY environment variable must be set");

        let icp_agent_endpoint = env::var("ICP_AGENT_ENDPOINT")
            .expect("ICP_AGENT_ENDPOINT environment variable must be set");

        let canister_principal_id = env::var("CANISTER_PRINCIPAL_ID")
            .expect("CANISTER_PRINCIPAL_ID environment variable must be set");

        let pinata_api_key =
            env::var("PINATA_API_KEY").expect("PINATA_API_KEY environment variable must be set");
        let pinata_api_secret = env::var("PINATA_API_SECRET")
            .expect("PINATA_API_SECRET environment variable must be set");
        let pinata_jwt =
            env::var("PINATA_JWT").expect("PINATA_JWT environment variable must be set");

        let pinata_ipfs_gateway = env::var("PINATA_IPFS_GATEWAY")
            .expect("PINATA_IPFS_GATEWAY environment variable must be set");

        return ENV {
            port,
            addrs,
            db_url,
            allowed_origns,
            secret_key,
            icp_agent_endpoint,
            canister_principal_id,
            pinata_api_key,
            pinata_api_secret,
            pinata_jwt,
            pinata_ipfs_gateway,
        };
    }
}
