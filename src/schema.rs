// @generated automatically by Diesel CLI.

diesel::table! {
    users (userid) {
        userid -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}
