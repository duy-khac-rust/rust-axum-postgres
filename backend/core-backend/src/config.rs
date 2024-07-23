use dotenv::var;
use serde::{Deserialize, Serialize};

use crate::error::ResultCore;

#[derive(Debug, Deserialize, Serialize)]
pub struct Web {
    pub port: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Postgres {
    pub dsn: String,
    pub max_connect: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DevConfig {
    pub devweb: Web,
    pub devpostgres: Postgres,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProdConfig {
    pub web: Web,
    pub postgres: Postgres,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DevEnv {
    pub app: DevConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProdEnv {
    pub app: ProdConfig,
}

impl ProdConfig {
    pub async fn from_env() -> ResultCore<Self> {
        match var("ENV").as_deref() {
            Ok("prod") => {
                let config: ProdEnv = config::Config::builder()
                    .add_source(config::Environment::default())
                    .build()?
                    .try_deserialize::<ProdEnv>()?;
                Ok(Self {
                    web: config.app.web,
                    postgres: config.app.postgres,
                })
            }

            _ => {
                let config: DevEnv = config::Config::builder()
                    .add_source(config::Environment::default())
                    .build()?
                    .try_deserialize::<DevEnv>()?;

                Ok(Self {
                    web: config.app.devweb,
                    postgres: config.app.devpostgres,
                })
            }
        }
    }
}
