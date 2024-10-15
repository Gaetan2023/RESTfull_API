// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        filepath -> Varchar,
    }
}
