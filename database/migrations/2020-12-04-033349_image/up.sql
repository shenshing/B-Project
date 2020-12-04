-- Your SQL goes here
Create Table images (
    serial_id   Serial Primary Key,
    img_id      Varchar(10),
    pro_type    Varchar(50),
    img_name    Text[],
    created_at  Timestamp Not Null Default Current_Timestamp
)