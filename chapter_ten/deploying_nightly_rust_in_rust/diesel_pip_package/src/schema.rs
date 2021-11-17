table! {
    alembic_version (version_num) {
        version_num -> Varchar,
    }
}
table! {
    fib_entries (id) {
        id -> Int4,
        input_number -> Nullable<Int4>,
        calculated_number -> Nullable<Int4>,
    }
}
allow_tables_to_appear_in_same_query!(
    alembic_version,
    fib_entries,
);
