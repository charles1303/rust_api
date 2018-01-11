#[derive(Queryable,RustcDecodable, RustcEncodable,Insertable)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String
}
