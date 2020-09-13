table! {
    users (id) {
        id -> Uuid,
        username -> Nullable<Varchar>,
        is_active -> Bool,
    }
}
