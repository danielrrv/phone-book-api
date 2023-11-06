use mongodb::{
    options::{
        ClientOptions, Compressor, Credential, ReadConcern, ServerAddress, ServerApi,
        ServerApiVersion, Tls,
    },
    Client,
};
use serde::Deserialize;
use std::{fs::read_to_string, path::Path, time::Duration};
use toml;

#[derive(Deserialize, Clone, Debug)]
struct ConnectionConfig {
    host: String,
    port: u16,
    //Note that by default, the driver will autodiscover other nodes in the cluster. To connect directly to a single server (rather than autodiscovering the rest of the cluster), set the direct_connection field to true.
    app_name: Option<String>,
    //The application name that the Client will send to the server as part of the handshake. This can be used in combination with the server logs to determine which Client is connected to a server.
    // compressors: Option<Vec<Compressor>>,""
    //The compressors that the Client is willing to use in the order they are specified in the configuration. The Client sends this list of compressors to the server. The server responds with the intersection of its supported list of compressors. The order of compressors indicates preference of compressors.

    //The connect timeout passed to each underlying TcpStream when attemtping to connect to the server.
    //The default value is 10 seconds.
    connect_timeout: Option<u64>,

    //The credential to use for authenticating connections made by this client.
    // credential: Option<Credentials>,

    //A value of zero indicates that there is no latency window, so only the server with the lowest average round trip time is eligible.
    //The default value is 15 ms.
    max_idle_time: Option<u64>,
    //The maximum amount of connections that the Client should allow to be created in a connection pool for a given server. If an operation is attempted on a server while max_pool_size connections are checked out, the operation will block until an in-progress operation finishes and its connection is checked back into the pool.
    max_pool_size: Option<u32>,

    //The default value is 10.
    min_pool_size: Option<u32>,
    //The minimum number of connections that should be available in a server’s connection pool at a given time. If fewer than min_pool_size connections are in the pool, connections will be added to the pool in the background until min_pool_size is reached.

    //The default value is 0.
    //The maximum number of new connections that can be created concurrently.
    max_connecting: Option<u32>,
    // read_concern: Option<ReadConcern>,

    //The name of the replica set that the Client should connect to.
    retry_reads: Option<bool>,
    //Whether or not the client should retry a read operation if the operation fails.

    //The default value is true.
    retry_writes: Option<bool>,
    //Whether or not the client should retry a write operation if the operation fails.
    default_database: Option<String>,
    // Default database for this client.

    //By default, no default database is specified.
    // tls: Option<bool>, //The TLS configuration for the Client to use in its connections with the server.

    //By default, TLS is disabled.
}

impl From<&'static Path> for ConnectionConfig {
    fn from<'a>(file_path: &'static Path) -> Self {
        if !file_path.exists(){
            panic!("File doesn't exist.")
        }
        let config_contents = read_to_string(file_path).expect("Should have been able to read the file");
        let config: ConnectionConfig = match toml::from_str(&format!(r"#{}#", config_contents)) {
            Ok(_config) => _config,
            Err(error) => panic!("{:?}", error),
        };
        println!("{:?}", config);
        config
    }
}

impl Into<ClientOptions> for ConnectionConfig {
    fn into(self) -> ClientOptions {
        ClientOptions::builder()
            .hosts(vec![ServerAddress::Tcp {
                host: self.host,
                port: Some(self.port),
            }])
            .app_name(self.app_name)
            .connect_timeout(Duration::new(self.connect_timeout.unwrap(), 0))
            // .credential(Credential::new())
            .max_idle_time(Duration::new(self.max_idle_time.unwrap(), 0))
            .max_pool_size(self.max_pool_size)
            .min_pool_size(self.min_pool_size)
            .max_connecting(self.max_connecting)
            // .read_concern(self.read_concern)
            .retry_reads(self.retry_reads)
            .retry_writes(self.retry_writes)
            .default_database(self.default_database)
            // .tls(self.tls)
            .build()
    }
}

pub struct Connection {
    client: Option<mongodb::Client>,
    config: Option<ConnectionConfig>,
}
impl Default for Connection {
    fn default() -> Self {
        Self {
            client: None,
            config: None,
        }
    }
}

impl Connection {
    pub fn from_config(path: &'static Path) -> Self {
        let config = ConnectionConfig::from(path);
        let mut connection = Connection::default();
        connection.config = Some(config);
        connection
    }
    pub fn get_client(mut self) -> mongodb::error::Result<Option<Client>> {
        let client_options: ClientOptions = self.config.expect("Connnection config not set").into();
        let client = Client::with_options(client_options)?;
        self.client = Some(client);
        Ok(self.client)
    }
}




#[cfg(test)]
mod tests{
    use std::path::Path;
    use std::env;
    use crate::connection::ConnectionConfig;
    #[test]
    fn first_test(){
        assert_eq!(1, 1)
    }
    #[test]
    fn from_config_test(){
        println!("{:?}", env::current_dir().unwrap());
        let config_path = Path::new("./../configuration.log");
        let config = ConnectionConfig::from(config_path );
        
    }
}
