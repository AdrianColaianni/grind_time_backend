// @generated automatically by Diesel CLI.

diesel::table! {
    entries (id) {
        id -> Int4,
        user_name -> Varchar,
        location -> Int4,
        minutes -> Int4,
    }
}
