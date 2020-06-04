// #![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use] extern crate rocket;

extern crate serde;
use serde::{Serialize, Deserialize};

use rocket_contrib::json::Json;
#[derive(Deserialize, Serialize)]
pub struct QA {
    pub question: String,
    pub answer: String,
}

#[get("/question")]
pub fn question() -> Json<Vec<QA>> {
    let question1 = QA {
        question: String::from("Answer the first question"),
        answer: String::from("Answer1"),
    };

    let question2 = QA {
        question: String::from("Answer the second question"),
        answer: String::from("Answer1"),
    };

    let question3 = QA {
        question: String::from("Answer the third question"),
        answer: String::from("Answer1"),
    };

    let question4 = QA {
        question: String::from("Answer the fourth question"),
        answer: String::from("Answer1"),
    };

    let question5 = QA {
        question: String::from("Answer the fifth question"),
        answer: String::from("Answer1"),
    };


    let mut vec_question: Vec<QA> = Vec::new();
    vec_question.push(question1);
    vec_question.push(question2);
    vec_question.push(question3);
    vec_question.push(question4);
    vec_question.push(question5);
    Json(vec_question)
}




// use std::time::SystemTime;
// use rocket::{Request, Data, Outcome::*};
// use rocket::data::{self, FromDataSimple};

// pub struct Datetime {
//     datetime: std::time::SystemTime,
// }

// impl FromDataSimple for Datetime {
//     type Error = String;

//     fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
//         let now = SystemTime::now();
        
//         let new_date = Datetime {
//             datetime: now
//         };

//         Success(new_date)
//     }
// }

// #[post("/date", data = "<date>")]
// pub fn test_time(date: Datetime) -> &'static str {
//     &"This is date time"
// }


