use diesel::prelude::*;
use diesel::*;
use dotenvy::dotenv;
use std::env;
use chrono::NaiveDateTime;

pub mod schema;



#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub complete: bool,
    pub created_at: NaiveDateTime,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

fn main() {
    use self::schema::todos::dsl::*;

    let con = &mut establish_connection();
    let res = todos
        .limit(10)
        .select(Todo::as_select())
        .load(con)
        .expect("Error");

    println!("Displaying {} todos", res.len());
    for todo in res {
        println!("{}", todo.id);
        println!("{}", todo.title);
        println!("{}", todo.complete);
        println!("{}", todo.created_at);
    }
}
