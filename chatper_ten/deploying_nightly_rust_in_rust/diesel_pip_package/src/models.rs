use crate::schema::fib_entries;
use crate::schema::alembic_version;


#[derive(Queryable, Debug, Identifiable)]
#[primary_key(version_num)]
#[table_name="alembic_version"]
pub struct AlembicVersion {
    pub version_num: String,
}


#[derive(Queryable, Debug, Identifiable)]
#[table_name="fib_entries"]
pub struct FibEntry {
    pub id: i32,
    pub input_number: Option<i32>,
    pub calculated_number: Option<i32>,
}
