extern crate chrono;
extern crate serde;

use serde::{Deserialize, Serialize};

use std::time::SystemTime;
use super::schema::users;
#[derive(Insertable, Deserialize, PartialEq, Clone, Debug)]
#[table_name="users"]
pub struct User {
    pub user_name:      String,
    pub user_gender:    String,
    pub user_email:     String,
    pub user_password:  String,
    // pub create_date:    SystemTime,
    pub user_profile:   Option<String>,
    pub user_role:      Option<String>,
    pub phone_number:   String,
}

#[derive(Queryable, Deserialize, PartialEq, Debug, Serialize)]
pub struct _User {
    pub user_id:        i32,
    pub user_name:      String,
    pub user_gender:    String,
    pub user_email:     String,
    pub user_password:  String,
    pub create_date:    SystemTime,
    pub user_profile:   Option<String>,
    pub user_role:      Option<String>,
    pub phone_number:   String,
}

impl _User {
    pub fn new() -> _User {
        let time = SystemTime::now();
        _User {
            user_id:        0i32,
            user_name:      String::from("no result"),
            user_gender:    String::from("no result"),
            user_email:     String::from("no result"),
            user_password:  String::from("no result"),
            create_date:    time,
            user_profile:   Some(String::from("no result")),
            user_role:      Some(String::from("no result")),
            phone_number:   String::from("no reuslt")
        }
    }
}
use rocket::{Request, Data, Outcome::*};
use rocket::data::{self, FromDataSimple};


impl FromDataSimple for User {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
            // let now = SystemTime::now();
            let new_user = User {
                user_name:      String::from("username"),
                user_gender:    String::from("user gender"),
                user_email:     String::from("user email"),
                user_password:  String::from("user password"),
                // create_date:    now,
                user_profile:   Some(String::from("user profile")),
                user_role:      Some(String::from("user role")),
                phone_number:   String::from("023 322 233")
            };
        Success(new_user)
    }
}


/*****************************/
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
// use crate::get_user_by_name_password;
// extern crate userInfo;
// use super::userInfo::get_user_by_name_password;
// use userInfo::get_user_by_name_password;

// #[derive(Debug)]
// pub enum UserError {
//     NotFound,
//     InvalidToken,
// }

// use crate::token::decode_token;
// impl<'a, 'r> FromRequest<'a, 'r> for _User {
//     type Error = UserError;

//     fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
//         let send_token: Vec<_> = request.headers().get("token_key").collect();

//         let token_st: String = send_token[0].to_string();

//         let claim = decode_token(token_st);

//         let name = claim.claims.user_name;
//         let password = claim.claims.user_password;

//         if(claim.claims.aud == String::from("koompiPlay")) {
//             let user = get_user_by_name_password(name, password);

//             return Outcome::Success(user.unwrap());
//         } else {
//             return Outcome::Failure((Status::BadRequest, UserError::NotFound));
//         }
//     }
// }
// #[derive(Debug)]
pub struct ApiKey(String);

impl ApiKey {
    #[inline(always)]
    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
    Expired,
    BadCount,
}

use crate::token::decode_token;
pub struct Token(String);
fn is_valid_token(token: &str) -> bool {

    let claim = decode_token(token.to_string());

    if(claim.claims.aud == String::from("koompiPlay")) {
        return true;
    } else {
        return false;
    }

}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("token").collect();

        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if is_valid_token(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}

#[get("/sensitive")]
pub fn sensitive(key: ApiKey) -> &'static str {
    println!("key: {}", key.into_inner());
    // let st = format!("{:#?}", key);
    // println!("st: {}", st);
    "Sensitive Data"
}
/****************************/

#[derive(Deserialize)]
pub struct loginInfo {
    pub user_email: String,
    pub user_password: String,
}


impl FromDataSimple for loginInfo {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {

        let login_info = loginInfo {
            user_email:  String::from("ok"),
            user_password: String::from("ok"),
        };
        
        Success(login_info)
    }
}

#[derive(Deserialize, Debug)]
pub struct LoginInfo {
    pub user_email:     String,
    pub user_password:  String,
    pub log_type:           String
}

impl FromDataSimple for LoginInfo {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {

        let login_info = LoginInfo {
            user_email:  String::from("ok"),
            user_password: String::from("ok"),
            log_type: String::from("ok"),
        };
        
        Success(login_info)
    }
}



#[derive(Deserialize, Clone)]
pub struct updateItem {
    // pub token:          String,
    pub newName:        Option<String>,
    pub newPassword:    Option<String>,
    pub newProfile:     Option<String>,
    pub newRole:        Option<String>,
    pub newPhone:       Option<String>
}

impl FromDataSimple for updateItem {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {

        let update_info = updateItem {
            // token:  String::from("ok"),
            newName:        Some(String::from("ok")),
            newPassword:    Some(String::from("ok")),
            newProfile:     Some(String::from("ok")),
            newRole:        Some(String::from("ok")),
            newPhone:       Some(String::from("ok")),
        };
        
        Success(update_info)
    }
}

#[derive(Deserialize)]
pub struct test_img {
    pub path: String,
}

impl FromDataSimple for test_img {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {

        let new_img = test_img {
            path: String::from("default-path"),
        };

        Success(new_img)
    }
}



#[derive(Deserialize, Serialize)]
pub struct stringObj {
    pub string: String
}

impl FromDataSimple for stringObj {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {

        let new_obj = stringObj {
            string: String::from("default string object"),
        };

        Success(new_obj)
    }
}


use super::schema::test_users;
#[derive(Insertable, Deserialize, PartialEq, Clone, Debug)]
#[table_name="test_users"]
pub struct Test_Users {
    pub user_name:          String,
    pub user_external_id:   Option<String>,
    pub user_gender:        String,
    pub user_email:         Option<String>,
    pub user_password:      Option<String>,
    // pub create_date:    SystemTime,
    pub user_profile:       Option<String>,
    pub user_role:          Option<String>,
    pub phone_number:       Option<String>,
    pub login_type:         String
}

// use diesel::query_dsl::RunQueryDsl;
// use diesel::sql_types::*;
#[derive(Queryable, Deserialize, PartialEq, Debug, Serialize)]
// #[table_name="test_users"]
pub struct _Test_Users {
    // #[sql_type = "Integer"]
    pub user_id:            i32,
    // #[sql_type = "Varchar"]
    pub user_external_id:   Option<String>,
    // #[sql_type = "Varchar"]
    pub user_name:          String,
    // #[sql_type = "Varchar"]
    pub user_gender:        String,
    // #[sql_type = "Varchar"]
    pub user_email:         Option<String>,
    // #[sql_type = "Varchar"]
    pub user_password:      Option<String>,
    // #[sql_type = "Timestamp"]
    pub create_date:        SystemTime,
    // #[sql_type = "Varchar"]
    pub user_profile:       Option<String>,
    // #[sql_type = "Varchar"]
    pub user_role:          Option<String>,
    // #[sql_type = "Varchar"]
    pub phone_number:       Option<String>,
    // #[sql_type = "Varchar"]
    pub login_type:         String
}

//  user_id,user_external_id,user_name,user_gender,user_email,user_password,create_date         |                                                    user_profile                                                     | user_role | phone_number | login_type 
