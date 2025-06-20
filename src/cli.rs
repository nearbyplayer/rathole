use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum KeypairType {
    X25519,
    X448,
}

lazy_static! {
    static ref VERSION: &'static str =
        option_env!("VERGEN_GIT_SEMVER_LIGHTWEIGHT").unwrap_or(env!("VERGEN_BUILD_SEMVER"));
    static ref LONG_VERSION: String = format!(
        "\nBuild Timestamp:     {}\nBuild Version:       {}\nCommit SHA:          {:?}\nCommit Date:         {:?}\nCommit Branch:       {:?}\ncargo Target Triple: {}\ncargo Profile:       {}\ncargo Features:      {}\n",
        env!("VERGEN_BUILD_TIMESTAMP"),
        env!("VERGEN_BUILD_SEMVER"),
        option_env!("VERGEN_GIT_SHA"),
        option_env!("VERGEN_GIT_COMMIT_TIMESTAMP"),
        option_env!("VERGEN_GIT_BRANCH"),
        env!("VERGEN_CARGO_TARGET_TRIPLE"),
        env!("VERGEN_CARGO_PROFILE"),
        env!("VERGEN_CARGO_FEATURES")
    );
}

#[derive(Parser, Debug, Default, Clone)]
#[command(
    about,
    version = *VERSION,
    long_version = LONG_VERSION.as_str(),
    group(
        clap::ArgGroup::new("cmds")
            .required(true)
            .args(["config-path", "genkey"])
    )
)]
pub struct Cli {
    /// The path to the configuration file
    ///
    /// Running as a client or a server is automatically determined
    /// according to the configuration file.
    #[arg(
        long = "config-path",
        value_name = "CONFIG",
        value_parser,
        required = false
    )]
    pub config_path: Option<std::path::PathBuf>,

    /// Run as a server
    #[arg(long, short, group = "mode")]
    pub server: bool,

    /// Run as a client
    #[arg(long, short, group = "mode")]
    pub client: bool,

    /// Generate a keypair for the use of the noise protocol
    ///
    /// The DH function to use is x25519
    #[arg(long, value_enum, value_name = "CURVE")]
    pub genkey: Option<Option<KeypairType>>,
}
