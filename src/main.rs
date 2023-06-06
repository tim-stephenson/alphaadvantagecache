mod get_treasury_data;
use get_treasury_data::TreasuryBillRates;
use tower_http::cors::{Any, CorsLayer};

use axum::{
    routing::{get},
    http::{StatusCode, Method, HeaderValue},
    Json, Router,
};

use std::{net::SocketAddr};






#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    
    // let mut result = (SystemTime::now(), make_request() )

    // build our application with a route
    let app = Router::new()
        .route("/treasury_bill_rates", get(treasury_bill_rates_handler) )
          .layer(
        CorsLayer::new()
        .allow_origin("http://localhost:9500".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET])
    );

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}



async fn treasury_bill_rates_handler() -> Result<Json<TreasuryBillRates>, StatusCode> {
    match get_treasury_data::get_data().await {
        Ok(json) => Ok(json),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
