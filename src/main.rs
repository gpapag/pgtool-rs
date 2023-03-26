use std::io;

use log::info;

use internal::args::Args;
use internal::credentials::get_db_credentials;
use internal::logger::initialize_logger;

use crate::internal::postgres_client::PostgresClient;

mod internal;

fn main() -> io::Result<()> {
    let args = Args::parse_args();

    initialize_logger(args.verbose.log_level_filter());

    info!("starting up");

    let credentials = get_db_credentials(args.pass_file.as_path(), &args.env)?;

    let src_client = PostgresClient::new(&credentials).unwrap();

    info!("host: {:?}, port: {:?}, db: {:?}, user: {:?}, password: {:?}",
        credentials.host,
        credentials.port,
        credentials.db,
        credentials.user,
        credentials.password);

    Ok(())
}
