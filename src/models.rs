use super::schema::comments;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Comment {
    pub id: i32,
    pub article: String,
    pub message: String,
    pub author: String,
}

#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub article: &'a str,
    pub message: &'a str,
    pub author: &'a str,
}
