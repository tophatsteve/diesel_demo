// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        #[max_length = 200]
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
