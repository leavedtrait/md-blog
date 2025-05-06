mod handlers;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, dev::Server, web::Data};
use tera::Tera;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,

            Err(e) => {
                println!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
        };

        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

pub fn start_srv() -> Result<Server, std::io::Error> {
    let srv = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(TEMPLATES.clone()))
            .service(Files::new("/static", "static/"))
            .wrap(Logger::new("%a %{User-Agent}i %D"))
            .service(handlers::home)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    Ok(srv)
}
