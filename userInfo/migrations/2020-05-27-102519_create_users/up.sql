-- Your SQL goes here
Create Table test_users (
    user_id                 Serial Primary Key,     /*not null*/
    user_external_id        Varchar,                /*nullable*/
    user_name               Varchar(20) Not Null,   /*not null*/
    user_gender             Varchar(10) Not Null,   /*not null*/
    user_email              Varchar(100) Unique,    /*nullable because if we login with telegram we don't have email*/                
    user_password           Varchar(100),           /*nullable because if we login with facwebook or google no password provide*/
    create_date             Timestamp Not Null Default Current_Timestamp,
    user_profile            Varchar, /*change to not null if facebook, google, telegram can give user_profile*/
    user_role               Varchar(10) Default 'User',   /*not null because event what type of login user use user still must have role*/
    phone_number            Varchar(20),    /*nullable because we not sure that facebook can give user phone number if they login via that*/
    login_type              Varchar(20) Not Null   /*change login_type to enum['local', 'facebook', 'google', 'gmail', 'telegram']*/
)