

// pub async fn backend_actix() -> {

//      HttpServer::new(|| {
//         App::new()
//             .service(hello)  // Register the hello endpoint  // Register the info endpoint
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

// #[get("/hello")]
// async fn hello() -> impl Responder {
//     "Hello from actix_web!"
// }