use clap::ArgMatches;

use meteora_client::kv::client::KVClient;

use crate::log::set_logger;

pub fn run_set_cli(matches: &ArgMatches) -> Result<(), std::io::Error> {
    set_logger();

    let server = matches.value_of("SERVER").unwrap();
    let key = matches.value_of("KEY").unwrap();
    let value = matches.value_of("VALUE").unwrap();

    let mut kv_client = KVClient::new(server);

    kv_client.set(key.to_string(), value.to_string())
}
