use diesel::pg::PgConnection;
use diesel::r2d2::{Pool, PooledConnection, ConnectionManager};
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, State};
use rocket::http::Status;
use rocket::async_trait;
use std::ops::{Deref,DerefMut};

pub type DBPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url : &str) -> DBPool{
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create a pool.")
}

pub struct Conn(pub PooledConnection<ConnectionManager<PgConnection>>);

#[async_trait]
impl<'r> FromRequest<'r> for Conn{
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self,()>{
        match request.guard::<&State<DBPool>>().await{
            Outcome::Success(pool) => match pool.get(){
                Ok(conn) => Outcome::Success(Conn(conn)),
                Err(_) => Outcome::Error((Status::ServiceUnavailable,()))
            },
            Outcome::Error(_) => Outcome::Error((Status::ServiceUnavailable,())),
            Outcome::Forward(_) => Outcome::Forward(Status::InternalServerError)
        }
    }
}

impl Deref for Conn{
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target{
        &self.0
    }
}

impl DerefMut for Conn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}