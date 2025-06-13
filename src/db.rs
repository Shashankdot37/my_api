use diesel::pg::PgConnection;
use diesel::r2d2::{self,ConnectionManager};
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::outcome::{Outcome, Success, Failure, Forward};
use rocket::{Request,State};
use std::ops::Deref;
use rocket::async_trait;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url:&str) -> Pool{
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool")
}

pub struct Conn(r2d2::PooledConnection<ConnectionManager<PgConnection>>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Conn{
    type Error=(Status,());

    async fn from_request(request:&'r Request<'_>)->Outcome<Self,Self::Error,()>{
        let pool = request.guard::<&State<Pool>>().await;
        match pool{
            Success(p) => match p.get(){
                Ok(conn) => Success(Conn(conn)),
                Err(_)=>Failure((Status::ServiceUnavailable, ())),
            },
            Failure(err) => Failure(err),
            Forward(f) => Forward(f),
        }
    }
}

impl Deref for Conn{
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target{
        &self.0
    }
}