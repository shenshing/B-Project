-- Your SQL goes here
Create Table products (
    pro_id             Varchar(10) Primary Key,
    pro_type           Varchar(50),
    pro_description    Varchar(100),
    pro_quantity       Integer,
    created_at         Timestamp Not Null Default Current_Timestamp
)