/// Configuration for the application.
#[derive(Debug)]
pub struct Config {
        /// The port to listen on. Defaults to 9090. Can be overridden with the `DG_PORT` environment
    /// variable.
    pub port: u16,
    /// The HDFS URI to connect to. Defaults to `http://localhost:50070/webhdfs/v1/data/deitos`. Can be overridden with the
    /// `HDFS_URI` environment variable.
    pub hdfs_uri: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: envmnt::get_u16("DV_PORT", 3030),
            hdfs_uri: envmnt::get_or("HDFS_URI", "http://localhost:50070/webhdfs/v1/data/deitos"),

        }
    }
}