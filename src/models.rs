use super::schema::comments;

#[derive(Queryable, Serialize)]
pub struct Comment {
    pub article: String,
    pub message: String,
    pub author: String,
    pub date: String,
    pub uuid: String,
}

#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub article: &'a str,
    pub message: &'a str,
    pub author: &'a str,
    pub date: &'a str,
    pub uuid: &'a str,
}

impl<'a> NewComment<'a>{
    pub fn to_comment(&self) -> Comment{
        Comment {
            article: self.article.to_owned(),
            message: self.message.to_owned(),
            author: self.author.to_owned(),
            date: self.date.to_owned(),
            uuid: self.uuid.to_owned(),
        }
    }
}
