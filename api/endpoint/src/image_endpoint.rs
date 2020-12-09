use actix_web::{post, get, HttpResponse, web, Error};
use actix_multipart::Multipart;
use std::io::Write;
use futures::{StreamExt, TryStreamExt};
use http::StatusCode;

use crate::id_generator::random_id;


#[post("/uploadimg")]
async fn upload_img(mut payload: Multipart) -> Result<HttpResponse, Error> {
// #[post("/uploadimg")]
//     async fn upload_img(mut payload: Multipart) -> HttpResponse {
    let mut file_name = String::new();
    while let Ok(Some(mut field)) = payload.try_next().await {
        
        // let content_type = field.content_disposition().unwrap();
        // let filename = content_type.get_filename().unwrap();
        let filename = random_id(5 as usize);
        let filepath = format!("./tmp/{}.jpeg", filename);
        file_name = filename.to_string();

        // println!("content_type: {}", content_type);
        // println!("filename: {}", filename);
        // println!("filepath: {}", filepath);
        
        // let filename = random_id(5 as usize);
        // let filepath = format!("./tmp/{}", filename);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    // Ok(HttpResponse::Ok().into())
    return Ok(
        HttpResponse::Ok()
            .content_type("application/json")
            .status(StatusCode::OK)
            .body(file_name)
    )
}

// use std::fs::File;
// use std::io::Read;
// use crate::id_generator::PasteID;
// #[get("/getimg/<id>")]
// // pub fn get_img(id: PasteID<'_>) -> Result<RawResponse, &'static str> {
// async fn get_img(id: PasteID<'_>) {
//     let file_format = format!("./tmp/{id}", id = id);
//     println!("file_fmt get: {}", file_format);
//     let mut file = File::open(file_format).unwrap();

//     let mut buffer = Vec::new();
//     file.read_to_end(&mut buffer).unwrap();
//     println!("{:?}", buffer);
//     // let name = String::from("a");
//     // Ok(RawResponse::from_vec(buffer, Some(name), Some(mime::IMAGE_STAR)))
// }

// use actix_files::NamedFile;
// use actix_web::{HttpRequest, Result};
use actix_web::Result;
// use std::path::PathBuf;

// #[get("/")]
// async fn index(req: HttpRequest) -> Result<NamedFile> {
//     let path: PathBuf = req.match_info().query("/home/shing/Documents/b-project/api/endpoint/tmp/17.png").parse().unwrap();
//     Ok(NamedFile::open(path)?)
// }

/*
#[get("/favicon")]
async fn favicon() -> Result<actix_files::NamedFile> {
    println!("inside favicon");
    Ok(actix_files::NamedFile::open("./tmp/17.png")?)
}

#[get("/favicon1/{id}")]
async fn favicon_(id: web::Path<(String,)>) -> Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open(format!("./tmp/{}", id.into_inner().0))?)
}
*/

#[get("/getimg/{id}")]
async fn get_image(id: web::Path<(String,)>) -> Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open(format!("./tmp/{}.jpeg", id.into_inner().0))?)
}