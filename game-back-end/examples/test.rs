#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate game_back_end;

use game_back_end::qa::static_rocket_route_info_for_question;
use game_back_end::datetime::static_rocket_route_info_for_test_time;
use game_back_end::user::static_rocket_route_info_for_save_player_data;
use game_back_end::datetime::static_rocket_route_info_for_return_players;
use game_back_end::datetime::static_rocket_route_info_for_return_top_scorer;
use game_back_end::datetime::static_rocket_route_info_for_return_winner;

use game_back_end::datetime::{Duration, query_top_scorer, find_winner};
use game_back_end::models::{PlayerQue};

extern crate rocket_cors;

extern crate userInfo;
// use userInfo::get_user_by_name_password;


fn main() {

    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();

    rocket::ignite()
        .mount("/", routes![question])
        .mount("/", routes![test_time])
        .mount("/", routes![save_player_data])
        .mount("/", routes![return_players])
        .mount("/", routes![return_top_scorer])
        .mount("/", routes![return_winner])   
        .attach(cors)
        .launch();


    // let name = String::from("shing");
    // let password = String::from("123");

    // let result = get_user_by_name_password(name, password).unwrap();

    // println!("{:#?}", result);        
}
