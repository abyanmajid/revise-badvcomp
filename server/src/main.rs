use revise_badvcomp::{cli, constants::ASCII_ART, logger, route::create_router};

use anyhow::Result;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use clap::Parser;
use cli::{CommandLines, SubCommands};
use tower_http::cors::CorsLayer;
use tracing::{debug, error, info, trace};

async fn serve(listen: bool, require_logging: bool, args: CommandLines) -> Result<()> {
    trace!(
        "Initializing serve function with parameters: listen={}, require_logging={}",
        listen,
        require_logging
    );
    let verbosity = if args.debug { 1 } else { args.verbosity };
    let level = logger::verbosity_to_level_filter(verbosity);
    let sub = logger::setup_logger(level)?;

    match tracing::subscriber::set_global_default(sub) {
        Ok(_) => trace!("Logging and tracing setup successfully."),
        Err(e) => error!("Failed to set global default subscriber: {}", e),
    }

    let SubCommands::ServeCommand(serve_args) = args.subcommand;
    debug!("Parsed subcommand arguments: {:?}", serve_args);

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_origin(
            "https://revise-badvcomp.vercel.app"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router().layer(cors);
    trace!("Router created with Axum.");

    if listen {
        let address = format!("{}:{}", serve_args.listener_ip, serve_args.listener_port);
        debug!("Attempting to bind to address: {}", address);
        info!("Starting server on {}", address);

        match tokio::net::TcpListener::bind(&address).await {
            Ok(listener) => {
                println!("{}\n🚀 Server is now live at {}\n", ASCII_ART, &address);
                info!("Server started, listening on {}", address);
                axum::serve(listener, app).await?;
            }
            Err(e) => {
                error!("Failed to bind to {}: {}", address, e);
                return Err(e.into());
            }
        }
    } else {
        debug!("Listening is disabled. Server will not start.");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CommandLines::parse();
    debug!("Parsed command line arguments: {:?}", args);
    serve(true, true, args).await
}
