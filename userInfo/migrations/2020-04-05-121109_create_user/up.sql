-- Your SQL goes here
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    user_name VARCHAR(20) NOT NULL,
    user_gender Varchar(10) NOT NULL,
    user_email VARCHAR(50) UNIQUE NOT NULL,
    user_password VARCHAR(100) NOT NULL,
    create_date TIMESTAMP NOT NULL Default Current_Timestamp,
    user_profile VARCHAR(1000),
    user_role VARCHAR(10) Default 'User',
    phone_number Varchar(20) NOT NULL
)