use rust_diesel::{models::Todo, *};
use diesel::prelude::*;

fn main() {
    let connection = create_connection();
    // let get_todos = posts.filter(published.eq(true))
    // ...って書けるみたいなんだけど、マイグレーションしたのにschemaに何も追加されない件について
}