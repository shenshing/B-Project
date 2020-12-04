table! {
    images (img_id) {
        img_id -> Varchar,
        pro_type -> Nullable<Varchar>,
        img_name -> Nullable<Array<Text>>,
        created_at -> Timestamp,
    }
}

table! {
    products (pro_id) {
        pro_id -> Varchar,
        pro_type -> Nullable<Varchar>,
        pro_description -> Nullable<Varchar>,
        pro_quantity -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    images,
    products,
);
