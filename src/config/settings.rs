use std::env;

pub struct ENV {
    pub port: u16,
    pub addrs: String,
    pub db_url: String,
    pub allowed_origns: String,
    pub secret_key: String,
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

        return ENV {
            port,
            addrs,
            db_url,
            allowed_origns,
            secret_key,
        };
    }
}
