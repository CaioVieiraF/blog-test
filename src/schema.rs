// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Text,
        title -> Text,
        body -> Text,
        public -> Bool,
        draft -> Bool,
        user_id -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        password -> Text,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
