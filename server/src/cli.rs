use clap::{arg, command, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    author("https://github.com/abyanmajid"),
    version,
    about = "Public API for generating BAdvComp practice problems.",
    long_about = "Public API written in Axum\n\n\
    with simple Next.js UI for generating.\n\
    practice problems to help in revising USYD's\n
    Bachelor of Advanced Computing units."
)]
#[derive(Debug)]
pub struct CommandLines {
    #[command(subcommand)]
    pub subcommand: SubCommands,
    /// Increase logging verbosity
    #[arg(short('v'), long, action = clap::ArgAction::Count)]
    pub verbosity: u8,
    /// Enable debug output, another way to increase logging verbosity
    #[arg(
        long = "debug",
        env = "API_DEBUG",
        value_name = "boolean",
        default_value_t = false
    )]
    pub debug: bool,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    #[command(
        about = "Start server",
        long_about = "Start the server listening on the configured address and port",
        name = "serve"
    )]
    ServeCommand(ServeArgs),
}

#[derive(Args, Debug)]
pub struct ServeArgs {
    /// IP address to listen on
    #[arg(
        short('i'),
        long = "ip",
        env = "API_IP",
        value_name = "address",
        default_value = "0.0.0.0"
    )]
    pub listener_ip: String,

    /// TCP port to listen on
    #[arg(
        short('p'),
        long = "port",
        env = "API_PORT",
        value_name = "tcp",
        default_value_t = 8000
    )]
    pub listener_port: u16,

    /// Test printing error message
    #[arg(
        short('t'),
        long = "test-error",
        value_name = "boolean",
        default_value_t = false
    )]
    pub test_err: bool,
}
