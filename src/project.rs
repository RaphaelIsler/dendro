#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Id{id: uuid::Uuid}


#[cfg(feature="backend")]
impl Id{
    pub fn from_str(id: &str) -> anyhow::Result<Self>{
        use std::str::FromStr;
        let id = uuid::Uuid::from_str(id)?;
        Ok(Self{id})
    }
}

impl Id{
    pub fn empty() -> Self{
        Self{id: uuid::Uuid::nil()}
    }
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Measurement{
    name: String,
    comment: String,
    x: i32,
    y: i32,
    distance: f64,
}



#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Section{
    all: Vec<Measurement>,
}


#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Path{
    sections: Vec<Section>,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct Image{
    width: i32,
    height: i32,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Project{
    info: Info,
    image: Image,
    path: Path
}

#[cfg(feature="backend")]
mod backend_m{
    use super::*;
    impl From<crate::osm4::Measurement> for Measurement{
        fn from(value: crate::osm4::Measurement) -> Self {
            Self { name: value.name, comment: value.comment, x: value.x, y: value.x, distance: value.distance }
        }
    }


    impl From<crate::osm4::PathIndex> for Section{
        fn from(value: crate::osm4::PathIndex) -> Self {
            Self { all: value.measurements.iter().map(|x| (x.clone()).into()).collect()}
        }
    }


    impl From<crate::osm4::Path> for Path{
        fn from(value: crate::osm4::Path) -> Self {
            Self { sections: value.paths.iter().map(|x| (x.clone()).into()).collect()}
        }
    }


    impl From<crate::osm4::Osm4> for Project{
        fn from(value: crate::osm4::Osm4) -> Self {
            Self {
                info: Info {
                    id: Id::empty(),
                    name: "Demo".to_string(),
                },
                image: Image { width: 5390, height: 688 },
                path: value.project_data.path.into()
            }
        }
    }

    impl Project{
        pub fn new_demo() -> Self{
            Self {
                info: Info {
                    id: Id::empty(),
                    name: "Demo".to_string(),
                },
                image: Image { width: 5390, height: 688 },
                path: Path{
                    sections: vec!(),
                },

            }
        }
    }
}
#[cfg(feature="backend")]
pub use backend_m::*;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Info{
    id: Id,
    name: String,
}

#[cfg(feature="frontend")]
mod frontend_m{
    use super::*;

    use yew::prelude::*;

    #[derive(Properties, PartialEq)]
    pub struct Props {
        pub id: Id,
    }


    impl Info{
        fn image_path(&self) -> String{
            "/image?".to_string() + &self.id.id.to_string()
        }
    }


    #[function_component]
    pub fn ProjectComp(Props{id} : &Props) -> Html {
        let loader = use_state(|| None::<Project>);
        let l2 = loader.clone();
        use std::ops::Deref;
        if let Some(project) = l2.deref(){
            html! {
                <svg width={project.image.width.to_string()} height = {project.image.height.to_string()} >
                    <image href={project.info.image_path()} width={project.image.width.to_string()} height = {project.image.height.to_string()} />
                    //<PathCmp path={project.path.clone()} />
                </svg>
            }
        } else {
            let id = id.clone();
            let loader = loader.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let request = crate::interface::ToBackend::GetProject { id: id.clone() } ;
                let response = request.request().await;
                if let Some(crate::interface::ToFrontend::ProjectLoaded { project }) = response {
                    loader.set(Some(project));
                }
            });
            html!{{"Warte auf daten"}}
        }
    }

}

#[cfg(feature="frontend")]
pub use frontend_m::*;