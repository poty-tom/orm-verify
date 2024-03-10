use diesel::prelude::*;

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub status: String,
}

// dieselのルールでstruct側は単数形で定義する
// 一方のDB側は複数系で定義する