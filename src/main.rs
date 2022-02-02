use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer
};
use command::*;
use state::*;
use std::sync::{Arc, Mutex};
use structopt::StructOpt;
use subxt::ClientBuilder;
use actix_web_middleware_keycloak_auth::{
    AlwaysReturnPolicy, DecodingKey, KeycloakAuth,
};
#[subxt::subxt(runtime_metadata_path = "sugarfunge_metadata.scale")]
pub mod sugarfunge {}
mod account;
mod asset;
mod command;
mod currency;
mod dex;
mod escrow;
mod state;
mod util;
mod user;

const KEYCLOAK_PK: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAgjxDaoGghFwAkdoo8YqoF4rVhZVmbkNTXrqDba47muKCnaULzlzOK2n//bB9Twaa/yxZ0cwli2vqsci1cNKQNh3zZjlLjeK6lEc/iDQvPLXad8/rRqj3ZgH+01YscOZBGdVq2GAOL+WYr3bhLD6yNiUOHXJQYrRoekfMYiQRmvV+c1/eXjFEbcqwOxKGxZ6CPIwWCEjPjwW2Hp8E4Ap518bzlKie491OJ9bkjAGf/6qhM/faf7Sx99Bhq8tk/d1fVZSCkW+MP+by/EyAruOS/0KEzHU6ERSp6gtoQ9AFYdYSv/J5/fYzWnuDemTWOy7GmUrdJI8D1CDmNKVgdYPDFwIDAQAB
-----END PUBLIC KEY-----";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let opt = Opt::from_args();

    let api = ClientBuilder::new()
        .set_url(opt.node_server.to_string())
        .build()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
        .to_runtime_api::<sugarfunge::RuntimeApi<sugarfunge::DefaultConfig>>();

    let state = AppState {
        api: Arc::new(Mutex::new(api)),
    };

    HttpServer::new(move || {
        let keycloak_auth = KeycloakAuth {
            detailed_responses: true,
            passthrough_policy: AlwaysReturnPolicy,
            keycloak_oid_public_key: DecodingKey::from_rsa_pem(KEYCLOAK_PK.as_bytes()).unwrap(),
            required_roles: vec![],
        };


        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(Data::new(state.clone()))
            .wrap(keycloak_auth)
            .route("user/verify_seed", web::get().to(user::verify_seed))
            .route("account/create", web::post().to(account::create))
            .route("account/fund", web::post().to(account::fund))
            .route("account/balance", web::get().to(account::balance))
            .route("asset/create_class", web::post().to(asset::create_class))
            .route("asset/create", web::post().to(asset::create))
            .route("asset/mint", web::post().to(asset::mint))
            .route("asset/burn", web::post().to(asset::burn))
            .route("asset/balance", web::get().to(asset::balance))
            .route("asset/transfer_from", web::post().to(asset::transfer_from))
            .route("currency/issue", web::post().to(currency::issue))
            .route("currency/issuance", web::get().to(currency::issuance))
            .route("currency/mint", web::post().to(currency::mint))
            .route("currency/burn", web::post().to(currency::burn))
            .route("currency/supply", web::get().to(currency::supply))
            .route("currency/balance", web::get().to(currency::balance))
            .route("dex/create", web::post().to(dex::create))
            .route("dex/buy_assets", web::post().to(dex::buy_assets))
            .route("dex/sell_assets", web::post().to(dex::sell_assets))
            .route("dex/add_liquidity", web::post().to(dex::add_liquidity))
            .route(
                "dex/remove_liquidity",
                web::post().to(dex::remove_liquidity),
            )
            .route("escrow/create", web::post().to(escrow::create_escrow))
            .route("escrow/refund", web::post().to(escrow::refund_assets))
            .route("escrow/deposit", web::post().to(escrow::deposit_assets))
    })
    .bind((opt.listen.host_str().unwrap(), opt.listen.port().unwrap()))?
    .workers(1)
    .run()
    .await
}