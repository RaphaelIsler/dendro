use actix_web::{ App, HttpServer, web, post, get, Responder, HttpResponse};
use anyhow::Result;
use std::io::Read;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};


#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Config{
    pub http_port: u32,
    pub https_port: u32,
    pub static_path: String,
    pub ssl_path: String,
    pub store_path: String,
}

#[cfg(feature="backend")]
impl Config{
    pub fn load_from_file(path: &str) -> anyhow::Result<Config>{
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(serde_json::from_str::<Self>(&contents)?)
    }
}

pub struct Server{
    config: Config,
}

//#[derive(Clone)]
pub struct AppData{
    service: crate::Service,
}

impl AppData{
}


#[get("/image")]
async fn get_image(data: web::Data<AppData>) -> actix_web::Result<actix_files::NamedFile> {
    match data.service.get_image_path().await{
        Ok(path) => {
            Ok(actix_files::NamedFile::open(path)?)
        }
        Err(err) => {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, err).into())
        }
    }
}



#[post("/control")]
async fn control(data: web::Data<AppData>, request: web::Json<crate::interface::ToBackend>) -> actix_web::Result<impl Responder>{
    let response = data.service.control(request.into_inner()).await;
    Ok(web::Json(response))
}


impl Server{
    pub fn new(config_path: &str) -> anyhow::Result<Box<Server>>{
        Ok(Box::new(Self{
            config: Config::load_from_file(config_path)?
        }))
    }


    pub async fn start(&mut self) -> Result<()>{
        let app_data = web::Data::new(AppData{
            service: crate::Service::new()
        });

        let (_,_) = futures::join!(
            {
                let address =  format!("{}:{}", "0.0.0.0", self.config.http_port.to_string());
                let static_path = self.config.static_path.clone();
                let app_data = app_data.clone();
                HttpServer::new(move || {
                    App::new()
                        .app_data(app_data.clone())
                        .service(get_image)
                        .service(control)
                        .service(actix_files::Files::new("/", std::path::PathBuf::from(static_path.clone()))
                            .use_last_modified(true)
                            .index_file("index.html".to_string()))

                })
                    .bind(address)?
                    .run()
            },
            {
                let address =  format!("{}:{}", "0.0.0.0", self.config.https_port.to_string());
                let static_path = self.config.static_path.clone();
                let key =  format!("{}{}", self.config.ssl_path, "cert.key");
                let cert =  format!("{}{}", self.config.ssl_path, "cert.crt");
                let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
                builder
                    .set_private_key_file(&key, SslFiletype::PEM)
                    .unwrap();
                builder.set_certificate_chain_file(&cert).unwrap();
                HttpServer::new(move || {
                    App::new()
                        .app_data(app_data.clone())
                        .service(get_image)
                        .service(control)
                        .service(actix_files::Files::new("/", std::path::PathBuf::from(static_path.clone()))
                            .use_last_modified(true)
                            .index_file("index.html".to_string()))

                })
                .bind_openssl(&address, builder)?
                .run()
            }
        );

        Ok(())
    }
}