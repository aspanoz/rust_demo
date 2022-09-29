#[macro_use]
extern crate log;

// use async_graphql::Schema;
// use async_graphql_warp::Response;
// use borovylo_data::{DataSchema, MutationRoot, QueryRoot, SubscriptionRoot};
// use std::convert::Infallible;
use borovylo_data::DataSchema;
use hyper::server::Server;
use listenfd::ListenFd;

use warp::{http::Response as HttpResponse, Filter};

pub async fn run(schema: DataSchema) {
    let index_page = warp::path::end().map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(include_str!("../public/index.html"))
    });

    let public_files = warp::path("public").and(warp::fs::dir("./web-client/public/"));

    let routes = warp::ws()
        .and(async_graphql_warp::graphql_protocol())
        .map(move |ws: warp::ws::Ws, protocol| {
            let schema = schema.clone();
            let reply = ws.on_upgrade(move |websocket| {
                async_graphql_warp::graphql_subscription_upgrade(websocket, protocol, schema)
            });
            warp::reply::with_header(
                reply,
                "Sec-WebSocket-Protocol",
                protocol.sec_websocket_protocol(),
            )
        })
        .or(public_files)
        .or(index_page);

    // hot reload
    let svc = warp::service(routes);
    let make_svc = hyper::service::make_service_fn(|_: _| {
        let svc = svc.clone();
        async move { Ok::<_, std::convert::Infallible>(svc) }
    });
    let mut listenfd = ListenFd::from_env();
    let server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        Server::from_tcp(l).unwrap()
    } else {
        Server::bind(&([0, 0, 0, 0], 5000).into())
    };

    info!("listening on 0.0.0.0:5000");
    server.serve(make_svc).await.unwrap()
}
