use std::time::SystemTime;
use rocket::{Request, Data, Outcome::*};
use rocket::data::{self, FromDataSimple};
use crate::establish_connection;
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize)]
pub struct Datetime {
    datetime: SystemTime
}

impl FromDataSimple for Datetime {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        let now = SystemTime::now();
        
        let new_date = Datetime {
            datetime: now
        };

        Success(new_date)
    }
}

#[derive(Deserialize)]
pub struct Duration {
    pub start: String,
    pub end:   String
}

impl FromDataSimple for Duration {
    type Error  = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        let time_start = SystemTime::now();
        let time_end = SystemTime::now();

        let new_duration = Duration {
            start: String::from("default time start"),
            end: String::from("default time end")
        };

        Success(new_duration)
    }
}

//function part
use diesel::sql_query;
use diesel::query_dsl::RunQueryDsl;
use crate::models::PlayerQue;
pub fn query_player_by_date(duration: Duration) -> Result<Vec<PlayerQue>, String> {
    // let query_format = format!("Select * From players Where playdate between '{}' and '{}';", 
    //     convert_systemtime_to_string(duration.start), 
    //     convert_systemtime_to_string(duration.end));
    println!("start: {}", duration.start.clone());
    println!("end: {}", duration.end.clone());
    let query_format = format!("Select * From players Where playdate between '{}' and '{}';", duration.start, duration.end);

    let conn = establish_connection();

    match sql_query(query_format).get_results::<PlayerQue>(&conn) {
        Ok(ok) => return Ok(ok),
        Err(err) => return Err(format!("{}", err)),
    }
}

pub fn query_top_scorer(duration: Duration) -> Result<Vec<PlayerQue>, String> {
    let query_format = format!("Select * From players Where score = (Select Max(score) From players) 
        And playdate Between '{}' and '{}';", duration.start, duration.end);
    let conn = establish_connection();

    match sql_query(query_format).get_results::<PlayerQue>(&conn) {
        Ok(ok) => {
            return Ok(ok);
        },
        Err(err) => {
            return Err(format!("{}", err));
        }
    }
}


use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

pub fn find_winner(duration: Duration) -> Json<PlayerQue> {
    let query_format = format!("Select * From players Where score = (Select Max(score) From players) 
        And playdate Between '{}' and '{}';", duration.start, duration.end);
    let conn = establish_connection();


    let winner = PlayerQue {
        id: 1i32,
        playername: String::from("winner"),
        score: 100i32,
        playdate: SystemTime::now(),
        email: String::from("winner@gmail.com")
    };

    
    let mut res: Vec<PlayerQue> = sql_query(query_format)
        .get_results(&conn)
        .unwrap();

    let mut rng = thread_rng();

    let winner = res.choose(&mut rng);

    let win1 = winner.unwrap();
    let win = PlayerQue {
        id: win1.id,
        playername: win1.playername.clone(),
        score: win1.score,
        playdate: win1.playdate,
        email: win1.email.clone()
    };

    return Json(win);
}

//endpoint part
use crate::models::Player;
#[post("/player",  data="<data_json>")]
pub fn return_players(data_json: Json<Duration>) -> Json<Vec<PlayerQue>> {
    let vec_player = query_player_by_date(data_json.into_inner()).unwrap();

    // Json(vec)
    Json(vec_player)

}

#[post("/topscorer", data="<data_json>")]
pub fn return_top_scorer(data_json: Json<Duration>) -> Json<Vec<PlayerQue>> {

    let vec_top = query_top_scorer(data_json.into_inner()).unwrap();

    Json(vec_top)
} 






//add-on function
#[post("/date", data = "<date>")]
pub fn test_time(date: Datetime) -> &'static str {
    &"This is date time"
}

#[post("/winner", data = "<data_json>")]
pub fn return_winner(data_json: Json<Duration>) -> Json<PlayerQue> {
    find_winner(data_json.into_inner())
}

use chrono::offset::Utc;
use chrono::DateTime;
pub fn convert_systemtime_to_string(date: SystemTime) -> String {
    let datetime: DateTime<Utc> = date.into();
    let time_fmt = datetime.format("%Y-%m-%d").to_string();

    return time_fmt;
}