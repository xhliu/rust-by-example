use super::schema::posts;

#[derive(Queryable, Insertable, AsChangeset, Identifiable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
