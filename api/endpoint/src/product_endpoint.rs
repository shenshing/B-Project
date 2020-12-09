use actix_web::{post, get, HttpResponse, web, /*Error*/};
// use actix_multipart::Multipart;
// use std::io::Write;
// use futures::{StreamExt, TryStreamExt};

use database::structure::Product;
use database::connection::establish_connection;
use database::product_db_op::insert_new_product;

use http::StatusCode;

#[post("/uploadproduct")]
async fn upload_product(pro_info: web::Json<Product>) -> HttpResponse {
    let insert_result = insert_new_product(pro_info.into_inner(), &establish_connection());
    
    match insert_result {
        Ok(_) => return HttpResponse::Ok()
                            .content_type("application/json")
                            .status(StatusCode::OK)
                            .body("Product Upload Successful")
        ,
        Err(_) => return HttpResponse::Ok()
                            .content_type("application/json")
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body("Something went wrong with our service")
    }
}


#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("plain/text")
        .header("X-Hdr", "sample")
        .status(StatusCode::NOT_FOUND)
        .body("data")
}
