use std::time::SystemTime;
use super::schema::{images, products};
// use diesel::sql_types::{Integer, Varchar, Timestamp};

#[derive(QueryableByName, Debug)]
#[table_name="images"]
pub struct Images {
    pub img_id:             String,
    pub pro_type:           Option<String>,
    pub img_name:           Option<Vec<String>>,
    pub created_at:         SystemTime,    
}

#[derive(Insertable, Debug)]
#[table_name="images"]
pub struct Image {
    pub img_id:             String,
    pub pro_type:           String,
    pub img_name:           Vec<String>
}

#[derive(QueryableByName, Debug)]
#[table_name="products"]
pub struct Products {
    pub pro_id:             Option<String>,
    pub pro_type:           Option<String>,
    pub pro_description:    Option<String>,
    pub pro_quantity:       Option<i32>,
    pub created_at:         SystemTime,
}

#[derive(Insertable)]
#[table_name="products"]
pub struct Product {
    pub pro_id:             String,
    pub pro_type:           String,
    pub pro_description:    String,
    pub pro_quantity:       i32
}