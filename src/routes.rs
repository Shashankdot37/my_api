use rocket::serde::json::Json;
use serde_json::json;
use serde_json::Value;

use crate::db::Conn as DbConn;
use super::models::{Player,NewPlayer};

#[get("/players")]
pub fn index(mut conn:DbConn) -> Json<Value>{
    let player = Player::all(&mut conn);

    Json(json!({
        "status":200,
        "result":player, 
    }))
}

#[post("/players", data="<new_player>")]
pub fn new(mut conn:DbConn, new_player: Json<NewPlayer>)->Json<Value>{
    let insert_status = Player::insert(new_player.into_inner(), &mut conn);
    let all_players = Player::all(&mut conn);
    let result = all_players.first();

    Json(json!({
        "status": insert_status,
        "result": result
    }))
}

#[get("/players/<id>")]
pub fn show(mut conn:DbConn, id:i32)->Json<Value>{
    let result = Player::show(id, &mut conn);
    let status = if result.is_empty(){404} else{200};

    Json(json!({
       "status":status,
       "result": result.get(0)
    }))
}

#[post("/players/<id>", data = "<new_data>")]
pub fn update(mut conn:DbConn, new_data: Json<NewPlayer>, id: i32)->Json<Value>{
    let result = Player::update_by_id(id, &mut conn, new_data.into_inner());
    
    Json(json!({
        "status":if result{200} else{404},
        "result":null
    }))
}

#[delete("/players/<id>")]
pub fn delete(mut conn:DbConn, id:i32)->Json<Value>{
    let deleted = Player::delete_by_id(id, &mut conn);

    Json(json!({
        "status": if deleted{200} else{404},
        "result": null
    }))
}

#[get("/players/players/<pname>")]
pub fn player(pname:String,mut conn:DbConn)->Json<Value>{
    let info = Player::info_by_name(pname, &mut conn);
    let status = if info.is_empty(){404} else{200};
    Json(json!({
        "status":status,
        "result": info
    }))
}

// #[catch(404)]
// pub fn not_found()->Json<Value>{
//     Json(json!({
//         "status": "Error",
//         "reason": "Resource not found"
//     }))
// }
