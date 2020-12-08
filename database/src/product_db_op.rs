use crate::structure::Product;
use diesel::PgConnection;
use diesel::sql_query;
use diesel::prelude::*;

pub fn insert_new_product(pro: Product, connection: &PgConnection) -> Result<String, String> {
    let statement = format!("Insert Into products Values ('{}', '{}', '{}', '{}');", 
        pro.pro_id, pro.pro_type, pro.pro_description, pro.pro_quantity
    );
    println!("{}", statement.clone());
    match sql_query(statement.clone())
        .execute(connection) {
            Ok(_) => return Ok(format!("insert new item successful")),
            Err(err) => return Err(format!("Error: {}", err))
        }
}