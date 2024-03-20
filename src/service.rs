use crate::project;
use crate::interface::{ToBackend, ToFrontend};

pub struct Service{

}

impl Service{

    pub fn new() -> Self{
        Self{}
    }

    pub async fn get_image_path(&self) ->  anyhow::Result<String>{
        Ok("./local/678240.jpg".to_string())
    }

    pub async fn get_projects(&self) -> Vec<project::Project>{
        vec!()
    }

    pub async fn control(&self, cmd: ToBackend) -> ToFrontend{
        match cmd{
            ToBackend::GetProject { id } => {
                if let Ok(osm4) = crate::Osm4::load("./local/678240.osm4").await{
                    ToFrontend::ProjectLoaded {
                        project: osm4.into()}
                } else {
                    ToFrontend::ProjectLoaded {
                        project: crate::project::Project::new_demo() }

                }
            }
        }
    }
}