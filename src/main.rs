mod get_treasury_data;


use axum::{
    routing::{get, post},
    http::{StatusCode, Response},
    response::IntoResponse,
    Json, Router,
    extract::Path
};
use bytes::{Bytes, Buf};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr};

use std::time::{Duration, SystemTime};




#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    
    // let mut result = (SystemTime::now(), make_request() )

    // build our application with a route
    let app = Router::new()
        .route("/treasury_bill_rates", get());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

treasury_bill_rates








// basic handler that responds with a static string
// async fn root() -> &'static str {
//     "Hello, World!"
// }







// async fn get_market_index(Path(symbol): Path<String>) -> Json<MarketIndex>{
//     println!("GET request for /market_index/:symbol for symbol: {}", symbol);
//     return Json(MarketIndex { 
//         symbol: "spx".to_string(), 
//         full_name: "S&P 500".to_string(), 
//         updated: 1685477607620, 
//         value: 4536.5, 
//         percent_change: -1.5 }) ;
// }


// #[derive(Serialize)]
// struct MarketIndex {
//     symbol : String,
//     full_name : String,
//     updated : u64,
//     value : f32,
//     percent_change : f32,

// }
