#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use self::models::{Post, NewPost};

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn create_post<'a>(conn: &mut PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

fn read_posts(conn: &mut PgConnection) -> Vec<Post> {
    use schema::posts::dsl::*;

    posts
        .load::<Post>(conn)
        .expect("Error loading posts")
}

fn update_post(conn: &mut PgConnection, post_id: i32, new_title: &str, new_body: &str) {
    use schema::posts::dsl::{posts, title, body};

    diesel::update(posts.find(post_id))
        .set((title.eq(new_title), body.eq(new_body)))
        .execute(conn)
        .expect("Error updating post");
}

fn delete_post(conn: &mut PgConnection, post_id: i32) {
    use schema::posts::dsl::posts;

    diesel::delete(posts.find(post_id))
        .execute(conn)
        .expect("Error deleting post");
}

fn main() {
    let mut connection = establish_connection();

    // Create
    let new_post = create_post(&mut connection, "My first post", "This is the body of my first post");
    println!("Created post: {:?}", new_post);

    // Read
    let all_posts = read_posts(&mut connection);
    println!("All posts: {:?}", all_posts);

    // Update
    update_post(&mut connection, new_post.id, "Updated title", "Updated body");
    let updated_post = read_posts(&mut connection);
    println!("Updated post: {:?}", updated_post);

    // Delete
    delete_post(&mut connection, new_post.id);
    let remaining_posts = read_posts(&mut connection);
    println!("Remaining posts: {:?}", remaining_posts);
}
