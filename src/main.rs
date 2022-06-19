//Data base conection libraries
#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use tera::Tera;
use dotenv::dotenv;
use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool; //to create a pool of conections

//Actix Libraries 
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use self::models::{Post, NewPost, NewPostHandler};
use self::schema::posts;
use self::schema::posts::dsl::*;

#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder{
    let conn = pool.get().expect("Error to fetch the data base");

    match web::block(move || {posts.load::<Post>(&conn)}).await{
        Ok(data) => {

            let data = data.unwrap();
            
            let mut ctx = tera::Context::new();
            ctx.insert("posts", &data);

            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("index.html", &ctx).unwrap()
            )
            // return HttpResponse::Ok().body(format!("{:?}", data));
        },
        Err(err) => HttpResponse::Ok().body("Error to recibe the data")
    }
}

#[get("/blog/{blog_slug}")]
async fn get_post(
    pool: web::Data<DbPool>, 
    template_manager: web::Data<tera::Tera>,
    blog_slug: web::Path<String>
) -> impl Responder{
    let conn = pool.get().expect("Error to fetch the data base");

    let url_slug = blog_slug.into_inner();

    match web::block(move || {posts.filter(slug.eq(url_slug)).load::<Post>(&conn)}).await{
        Ok(data) => {

            let data = data.unwrap();
            if data.len()==0{
                return HttpResponse::NotFound().finish();
            }
            let data = &data[0];
            
            let mut ctx = tera::Context::new();
            ctx.insert("post", data);

            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("posts.html", &ctx).unwrap()
            )
            // return HttpResponse::Ok().body(format!("{:?}", data));
        },
        Err(err) => HttpResponse::Ok().body("Error to recibe the data")
    }
}

#[post("/new_post")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder{
    let conn = pool.get().expect("Error to fetch the database");

    println!("{:?}", item);

    match web::block(move || {Post::create_post(&conn, &item)}).await{
        Ok(data) => {
            return HttpResponse::Ok().body(format!("{:?}", data));
        },
        Err(err) => HttpResponse::Ok().body("Error to recibe the data")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("BD url don't fund");   

    let connection = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder().build(connection).expect("Can't build Pool");//Whit this we have acces to the database

    HttpServer::new(move ||{
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
        .service(index)
        .service(new_post)
        .service(get_post)
        .app_data(web::Data::new(pool.clone()))
        .app_data(web::Data::new(tera))
    }).bind(("localhost", 3000)).unwrap().run().await
}

    // let conn = PgConnection::establish(&db_url).expect("We can't connect to the data base");
    // use self::models::{Post, NewPost, PostSimplificade};
    // use self::schema::posts;
    // use self::schema::posts::dsl::*;

    // // Insert data
    // let new_post = NewPost{
    //     title:"My post"
    //     body: " Lorean..."
    //     slug: "n post"
    // };
    // let post: Post = diesel::insert_into(post::table).values(&new_post).get_result(&conn).expect("Insertion failed");

    // //Select * from posts
    // println!("Query without limits");
    // let posts_result = posts.load::<Post>(&conn).expect("Error to execute query");
    // for post in posts_result{
    //     println!("{:?}", post);
    // }


    
    // // Select title and body from post
    // println!("\nPost filtered by columns");
    // let posts_result = posts.select((title, body)).load::<PostSimplificade>(&conn).expect("Error executing query");
    // for post in posts_result{
    //     println!("{:?}", post);
    // }

    // // Query limited and ordered by id
    // println!("\nQuery limited and ordered by id");
    // let posts_result = posts.order(id.desc()).limit(2).load::<Post>(&conn).expect("Error executing query");
    // for post in posts_result{
    //     println!("{:?}", post);
    // }

    // // Query whit where
    // println!("\nQuery whit where");
    // let posts_result = posts.filter(slug.eq("first-post")).limit(2).load::<Post>(&conn).expect("Error executing query");
    // for post in posts_result{
    //     println!("{:?}", post);
    // }

// // To update records:
//     let post_update = diesel::update(posts.filter(id.eq(4))).set((slug.eq("fourth-post"), title.eq("My fourth blogpost"))).get_result::<Post>(&conn).expect("Error in update");

// To delete records
//     diesel::delete(posts.filter(slug.eq("fifth-post"))).execute(&conn).expect("Failed delete");

// diesel::delete(posts.filter(slug.like("%-post%"))).execute(&conn).expect("Deliting Failed ");