// Constants used across the project

use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

pub const TOKEN_PREFIX: &str = "Bearer";

lazy_static! {
    /// Database connection string loaded from the environment
    pub static ref DATABASE_URL: String = {
        dotenv().ok();
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    };

    /// Secret used to sign and verify access tokens
    pub static ref ACCESS_TOKEN: String = {
        dotenv().ok();
        env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN must be set")
    };

    /// Cost factor used by bcrypt when hashing passwords
    pub static ref BCRYPT_COST: u32 = {
        dotenv().ok();
        env::var("BCRYPT_COST")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
            .unwrap_or(12)
    };

    /// Lifetime in seconds for issued JWT tokens
    pub static ref TOKEN_EXPIRATION_SECS: usize = {
        dotenv().ok();
        env::var("TOKEN_EXPIRATION_SECS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(3600)
    };

    /// Directory where log files will be written
    pub static ref LOG_DIR: String = {
        dotenv().ok();
        env::var("LOG_DIR").unwrap_or_else(|_| "logs".into())
    };
}
