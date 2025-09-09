mod crypto;
mod steg;
mod net;
mod client;
mod server;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let mode = args.next().unwrap_or_else(|| {
        eprintln!("usage:\n  server <addr:port> <password>\n  client <server:port> <cover_image> <password> <secret_file_or_->");
        std::process::exit(1);
    });

    match mode.as_str() {
        "server" => {
            let addr = args.next().expect("missing addr");
            let password = args.next().expect("missing password");
            server::run(&addr, &password).await?;
        }
        "client" => {
            let addr = args.next().expect("missing server addr");
            let cover = args.next().expect("missing cover image");
            let password = args.next().expect("missing password");
            let secret = args.next().expect("missing secret file");
            let addr = addr.parse().expect("invalid addr");
            client::run(addr, &cover, &password, &secret).await?;
        }
        _ => {
            eprintln!("invalid mode");
        }
    }
    Ok(())
}
