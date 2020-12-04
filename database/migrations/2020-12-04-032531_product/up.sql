-- Your SQL goes here
Create Table products (
    serial_id          Serial Primary Key,
    pro_id             Varchar(10),
    pro_type           Varchar(50),
    pro_description    Varchar(100),
    pro_quantity       Integer,
    created_at         Timestamp Not Null Default Current_Timestamp
)