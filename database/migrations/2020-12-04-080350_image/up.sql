-- Your SQL goes here
Create Table images (
    img_id      Varchar(10) Primary Key,
    pro_type    Varchar(50),
    img_name    Text[],
    created_at  Timestamp Not Null Default Current_Timestamp
)