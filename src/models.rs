use diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket::serde::Serialize;
use rocket::serde::Deserialize;

use crate::schema::players;
use crate::schema::players::dsl::players as all_players;
//idea is to give the name and id and the api will return if the player is available for the match.
#[derive(Queryable,Serialize,Clone,Debug)]
pub struct Player{
    pub id: i32,
    pub pname: String,
    pub jersey_no: i32,
    pub available: bool
}


//used to create a new player object and insert to database
#[derive(Insertable,Serialize, Deserialize)]
#[diesel(table_name = players)]
pub struct NewPlayer{
    pub pname: String,
    pub jersey_no: i32,
    pub available: bool
}

impl Player{
    pub fn show(id:i32, conn: &mut PgConnection)->Vec<Player>{
        all_players.find(id).load::<Player>(conn).expect("Error loading player.")
    }

    pub fn all(conn: &mut PgConnection)->Vec<Player>{
        all_players.order(players::id.desc()).load::<Player>(conn).expect("Error loading player.")
    }

    pub fn update_by_id(id:i32, conn: &mut PgConnection, player:NewPlayer)->bool{
        use crate::schema::players::dsl::{pname as p,jersey_no as j, available as a};
        let NewPlayer{pname, jersey_no, available} = player;
        diesel::update(all_players.find(id)).set((p.eq(pname), j.eq(jersey_no), a.eq(available))).get_result::<Player>(conn).is_ok()
    }
    pub fn insert(player:NewPlayer, conn:&mut PgConnection)->bool{
        diesel::insert_into(players::table).values(&player).execute(conn).is_ok()
    }
    pub fn delete_by_id(id: i32, conn: &mut PgConnection)->bool{
        if Player::show(id, conn).is_empty(){
            return false;
        }
        else {
            diesel::delete(all_players.find(id)).execute(conn).is_ok()
        }
    }
    pub fn info_by_name(name:String, conn: &mut PgConnection)->Vec<Player>{
        all_players.filter(players::pname.eq(name)).load::<Player>(conn).expect("Error loading the information about player.")
    }
}