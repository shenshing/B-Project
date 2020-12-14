use database::image_db_op::read_img;
use database::connection::establish_connection;
use database::image_db_op::insert_new_image;
use database::structure::Image;

use std::env;

fn main() {
    // read_img(&establish_connection());

    // let img = Image {
    //     img_id: String::from("id2"),
    //     pro_type: String::from("clothes"),
    //     img_name:   [String::from("shirt1"), String::from("shirt2"), String::from("shirt3")].to_vec()
    // };
    // insert_new_image(img ,&establish_connection());

    // insert_new(img, &establish_connection());


    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    println!("database url: {}", database_url);

}