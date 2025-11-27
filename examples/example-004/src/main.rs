async fn handler_root() -> &'static str {
  "Welcome to Example 004!"
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  let args: Vec<String> = std::env::args().collect();
  let address = format!("0.0.0.0:{}", args[1]);
  let app = axum::Router::new().route("/", axum::routing::get(handler_root));
  let listener = match tokio::net::TcpListener::bind(address.clone()).await {
    Ok(listener) => listener,
    Err(e) if e.kind() == tokio::io::ErrorKind::AddrInUse => {
      eprintln!("address {} is already in use", address);
      std::process::exit(1);
    }
    Err(e) => {
      eprintln!("Unexpected error: {}", e);
      std::process::exit(2);
    }
  };
  println!("started server at address {}", address);
  axum::serve(listener, app).await.unwrap();
}
