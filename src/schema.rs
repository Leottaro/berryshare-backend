// @generated automatically by Diesel CLI.

diesel::table! {
    shares (id) {
        id -> Unsigned<Integer>,
        userid -> Integer,
        size -> Unsigned<Bigint>,
        creation -> Datetime,
        lifespan -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 32]
        password -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        admin -> Bool,
    }
}

diesel::joinable!(shares -> users (userid));

diesel::allow_tables_to_appear_in_same_query!(shares, users,);
