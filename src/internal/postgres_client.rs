//! postgres_client

use postgres::{Client, Config, Error, NoTls};

use crate::internal::credentials::Credentials;

pub struct PostgresClient {
    client: Client,
}

impl PostgresClient {
    pub fn new(cred: &Credentials) -> Result<PostgresClient, Error> {
        let client = Config::new()
            .host(&cred.host)
            .port(cred.port)
            .dbname(&cred.db)
            .user(&cred.user)
            .password(&cred.password)
            .connect(NoTls)?;

        //let client = config.connect(NoTls)?;

        let postgres_client = PostgresClient { client };

        Ok(postgres_client)
    }
}
