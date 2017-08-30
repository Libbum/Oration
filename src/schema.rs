table! {
    comments (id) {
        id -> Integer,
        tid -> Nullable<Binary>,
        parent -> Nullable<Integer>,
        created -> Float,
        modified -> Nullable<Float>,
        mode -> Integer,
        remote_addr -> Nullable<Text>,
        text -> Text,
        author -> Nullable<Text>,
        email -> Nullable<Text>,
        website -> Nullable<Text>,
        likes -> Nullable<Integer>,
        dislikes -> Nullable<Integer>,
        voters -> Text,
    }
}

table! {
    preferences (key) {
        key -> Text,
        value -> Text,
    }
}

table! {
    threads (id) {
        id -> Integer,
        uri -> Nullable<Text>,
        title -> Nullable<Text>,
    }
}
