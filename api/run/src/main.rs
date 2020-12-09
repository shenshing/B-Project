use actix_web::{get, App, HttpResponse, Responder};
use actix_web::{HttpServer, middleware};
use endpoint::product_endpoint::upload_product;
use endpoint::image_endpoint::{upload_img, get_image};
use endpoint::product_endpoint::index;



#[get("/")]
async fn testing_connection() -> impl Responder {
    HttpResponse::Ok().body("Server up and Running......")
}


// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("./tmp").unwrap();

    let ip = "0.0.0.0:3000";

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default())
            .service(testing_connection)
            .service(upload_product)
            .service(upload_img)
            .service(get_image)
    })
    .bind(ip)?
    .run()
    .await
}
