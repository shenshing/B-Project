use actix_web::{HttpServer, App, middleware};
use endpoint::product_endpoint::upload_product;
use endpoint::image_endpoint::{upload_img, get_image};
// use endpoint::image_endpoint::favicon_;
use endpoint::product_endpoint::index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("./tmp").unwrap();

    let ip = "0.0.0.0:3000";

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default())
            // .service(
            // // web::resource("/")
            //     // .route(web::get().to(index))
            //     // .route(web::post().to(upload_img)),
            //     // upload_img
            //     // index,
            //     upload_product,
            //     // upload_img
            // )
            // .service(favicon)
            // .service(favicon_)
            .service(upload_product)
            .service(upload_img)
            .service(get_image)
            .service(index)
    })
    .bind(ip)?
    .run()
    .await
}


// use endpoint::id_generator;

// fn main() {
//     let result = id_generator::random_id(5 as usize);
//     println!("{:#?}", result);
// }