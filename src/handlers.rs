use actix_web::{HttpResponse, Responder, get, web};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Serialize, Deserialize, Debug)]
pub struct Frontmatter {
    title: String,
    file_name: String,
    description: String,
    posted: String,
    tags: Vec<String>,
    author: String,
    estimated_reading_time: u32,
    order: u32,
}

#[get("/")]
async fn home(templates: web::Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("title", "Home | leavedtrait");
    let frontmatters = vec![
        Frontmatter {
            tags: vec!["Rusty".to_string(), "Test".to_string()],
    
            title: "Test posts".to_string(),
    
            file_name: "test_posts.md".to_string(),
    
            description: "Just testing out the system".to_string(),
    
            posted: "2022-08-09".to_string(),
    
            author: "MBvisti".to_string(),
    
            estimated_reading_time: 12,
    
            order: 1,
        },
        Frontmatter {
            tags: vec!["Rusty".to_string(), "Test".to_string()],
    
            title: "Test posts".to_string(),
    
            file_name: "test_posts.md".to_string(),
    
            description: "Just testing out the system".to_string(),
    
            posted: "2022-08-09".to_string(),
    
            author: "MBvisti".to_string(),
    
            estimated_reading_time: 12,
    
            order: 1,
        },
        Frontmatter {
        tags: vec!["Rusty".to_string(), "Test".to_string()],

        title: "Test posts".to_string(),

        file_name: "test_posts.md".to_string(),

        description: "Just testing out the system".to_string(),

        posted: "2022-08-09".to_string(),

        author: "MBvisti".to_string(),

        estimated_reading_time: 12,

        order: 1,
    }];

    ctx.insert("posts", &frontmatters);
    match templates.render("index.html", &ctx) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),

        Err(e) => {
            println!("{:?}", e);

            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>")
        }
    }
}