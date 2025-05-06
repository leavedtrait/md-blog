use env_logger::Env;
use md_blog::start_srv;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    match start_srv(){
        Ok(srv )=>{
            srv.await?;
            Ok(())
        }
        Err(e) => {
            eprintln!("Error could not start server: {:#?} ",e);
            Ok(())
        }
    }
}
