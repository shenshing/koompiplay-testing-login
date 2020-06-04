-- Your SQL goes here
Create Table players (
    id Serial Primary Key,
    playername Varchar(50) Not Null,
    score Integer Not Null,
    playdate Timestamp Not Null Default Current_Timestamp,
    email Varchar(50) Not Null
)