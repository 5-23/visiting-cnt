mod counter;

use axum::{routing::get, Router};
use shuttle_secrets::SecretStore;
use std::env::{var, set_var};

async fn hello_world() -> &'static str {
    let a = var("MONGO_KEY").unwrap();
    println!("{a}");
    "a"
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let mongo_key = secret_store.get("DB").unwrap();
    set_var("MONGO_KEY", mongo_key);
    
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/count", get(counter::count))
    ;
    Ok(router.into())
}
