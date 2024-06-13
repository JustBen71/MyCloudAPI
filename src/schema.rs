// @generated automatically by Diesel CLI.

diesel::table! {
    users (id_user) {
        id_user -> Integer,
        #[max_length = 255]
        login_user -> Varchar,
        #[max_length = 255]
        email_user -> Varchar,
        #[max_length = 255]
        password_user -> Varchar,
    }
}
