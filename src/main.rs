#[cfg(feature = "frontend")]
pub mod frontend;

#[cfg(feature = "frontend")]
pub use frontend::*;


#[cfg(feature = "frontend")]
fn main() {
    yew::Renderer::<App>::new().render();
}



#[cfg(feature = "backend")]
#[path ="."]
pub mod backend{
    pub mod server;
}

#[cfg(feature = "backend")]
pub use backend::*;

#[cfg(feature = "backend")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let config_path = &args[1];
    match server::Server::new(config_path){
        Ok(mut server) =>{
            match server.start().await{
                Ok(_) => Ok(()),
                Err(err) =>  {
                    println!("server error: {}", err);
                    Ok(())
                }
            }
        }
        Err(err) => {
            println!("could not load config {}: Exit! ({})", config_path, err.to_string());
            Ok(())
        }
    }
}
