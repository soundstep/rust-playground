// use actix_web::body::BoxBody;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use reqwest::header::ACCEPT;
// use std::sync::Arc;
// use std::future::Future;

// use reqwest::get;
// use chrono::{DateTime, Utc};

// #[derive(Clone)]
// pub struct RequestClient {
//     instance: Arc<reqwest::Client>,
// }

// impl RequestClient {
//     fn new() -> Self {
//         let client = reqwest::Client::new();
//         RequestClient {
//             instance: Arc::new(client),
//         }
//     }
// }

#[get("/")]
async fn hello() -> impl Responder {
    // let now: DateTime<Utc> = Utc::now();
    // println!("Received {}", now);
    HttpResponse::Ok().body("Hello world!")
}

// struct ForwardResponder {
//     client: RequestClient,
// }

// impl Responder for ForwardResponder {
//     type Body = BoxBody;

//     fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
//         // let body = serde_json::to_string(&self).unwrap();

//         // Create response and set content type
//         HttpResponse::Ok()
//             // .content_type(ContentType::json())
//             .body(&self.to_string())
//     }
// }

// #[get("/forward")]
// async fn forward() -> impl Responder {
//     // let now: DateTime<Utc> = Utc::now();
//     // println!("Received {}", now);
//     let url = "http://promoted.hubsvc.itv.com/promoted/itvonline/ctv/homepage/ctv?broadcaster=ITV&features=hls,aes,progressive,inband-webvtt&ishubplus=false&regionalBroadcaster=ITV";
//     let accept_header = "application/vnd.itv.hubsvc.promotion.v1+vnd.itv.hubsvc.programme.v3+vnd.itv.hubsvc.production.v3+vnd.itv.hubsvc.channel.v2+hal+json";
//     // let client = reqwest::Client::builder().build();
//     let client = reqwest::Client::new();
//     let res = client
//         .get(url)
//         .header(ACCEPT, accept_header)
//         .send()
//         .await
//         .unwrap();
//     println!("Status: {}", res.status());
//     // println!("Headers:\n{:#?}", res.headers());

//     let body = res.text().await.unwrap();
//     // println!("Body:\n{}", body);
//     HttpResponse::Ok()
//         .content_type("application/json")
//         .body(body)
// }

async fn forward() -> impl Responder {
    // let now: DateTime<Utc> = Utc::now();
    // println!("Received {}", now);
    let url = "http://promoted.hubsvc.itv.com/promoted/itvonline/ctv/homepage/ctv?broadcaster=ITV&features=hls,aes,progressive,inband-webvtt&ishubplus=false&regionalBroadcaster=ITV";
    let accept_header = "application/vnd.itv.hubsvc.promotion.v1+vnd.itv.hubsvc.programme.v3+vnd.itv.hubsvc.production.v3+vnd.itv.hubsvc.channel.v2+hal+json";
    // let client = reqwest::Client::builder().build();
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(ACCEPT, accept_header)
        .send()
        .await
        .unwrap();
    let res = client
        .get(url)
        .header(ACCEPT, accept_header)
        .send()
        .await
        .unwrap();
    let res = client
        .get(url)
        .header(ACCEPT, accept_header)
        .send()
        .await
        .unwrap();
    let res = client
        .get(url)
        .header(ACCEPT, accept_header)
        .send()
        .await
        .unwrap();
    let res = client
        .get(url)
        .header(ACCEPT, accept_header)
        .send()
        .await
        .unwrap();
    let res = client
        .get(url)
        .header(ACCEPT, accept_header)
        .send()
        .await
        .unwrap();
    let res = client
        .get(url)
        .header(ACCEPT, accept_header)
        .send()
        .await
        .unwrap();
    let res = client
        .get(url)
        .header(ACCEPT, accept_header)
        .send()
        .await
        .unwrap();
    let res = client
        .get(url)
        .header(ACCEPT, accept_header)
        .send()
        .await
        .unwrap();
    let res = client
        .get(url)
        .header(ACCEPT, accept_header)
        .send()
        .await
        .unwrap();
    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await.unwrap();
    // println!("Body:\n{}", body);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// async fn forward() -> impl Future<Item = String, Error = String> {
//     let url = "http://promoted.hubsvc.itv.com/promoted/itvonline/ctv/homepage/ctv?broadcaster=ITV&features=hls,aes,progressive,inband-webvtt&ishubplus=false&regionalBroadcaster=ITV";
//     let client = reqwest::Client::builder().build()?;
//     return reqwest::get(url);
// }

// async fn forward() -> HttpResponse {
//     // let url = "http://promoted.hubsvc.itv.com/promoted/itvonline/ctv/homepage/ctv?broadcaster=ITV&features=hls,aes,progressive,inband-webvtt&ishubplus=false&regionalBroadcaster=ITV";
//     // let client = reqwest::Client::builder().build();
//     // let res = client.get(url);
//     HttpResponse::Ok().body("gna")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            // .service(forward)
            .route("/forward", web::get().to(forward))
            .route("/forward", web::head().to(forward))
            .route("/hey", web::get().to(manual_hello))
        // .service(web::resource("/forward").to(forward))
        // .route("/forward", web::get().to_async(forward))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
