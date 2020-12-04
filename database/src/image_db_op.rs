use crate::structure::{Image, Images};
use diesel::PgConnection;
use diesel::sql_query;
use diesel::prelude::*;
// use crate::schema::images;

// insert into images values (1, 'id1', 'clothes', '{"a", "b"}');
pub fn insert_new_image(img: Image, connection: &PgConnection) -> Result<String, String> {
    let statement = format!("Insert Into images Values ('{}', '{}', '{{\"{}\", \"{}\", \"{}\"}}');", 
        img.img_id, img.pro_type, img.img_name[0], img.img_name[1], img.img_name[2]
    );
    println!("{}", statement.clone());
    match sql_query(statement.clone())
        .execute(connection) {
            Ok(_) => return Ok(format!("insert new image successful")),
            Err(err) => return Err(format!("Error: {}", err))
        }
}

// pub fn insert_new_image(img: Image, connection: &PgConnection) {
//     match diesel::insert_into(images::table)
//         .values(&img)
//         .execute(connection) {
//             Ok(_) => {
//                 println!("insert new image successful");
//             },
//             Err(err) => {
//                 println!("error saving new image {}", err);
//         }
//     }
// }

pub fn delete_img() {

}

pub fn update_img() {

}

pub fn read_img(connection: &PgConnection) -> Result<Vec<Images>, diesel::result::Error> {
    let statement = format!("Select * From images");

    match sql_query(statement)
        .load(connection) {
            Ok(imgs) => return Ok(imgs),
            Err(err) => return Err(err),
        }
}