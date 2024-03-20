#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Measurement{
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Comment")]
    pub comment: String,

    #[serde(rename = "XPOS")]
    pub x: i32,
    #[serde(rename = "YPOS")]
    pub y: i32,

    #[serde(rename = "Distance")]
    pub distance: f64,
}



#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct PathIndex{
    #[serde(rename = "$value")]
    pub measurements: Vec<Measurement>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Path{
    #[serde(rename = "Index")]
    pub paths: Vec<PathIndex>
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct ProjectData{
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Path")]
    pub path: Path,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Osm4{
    #[serde(rename = "ProjectData")]
    pub project_data: ProjectData,
}

impl Osm4{
    pub async fn load(path: &str) -> anyhow::Result<Self>{
        let content = tokio::fs::read_to_string(path).await?;
        Ok(serde_xml_rs::from_str(&content)?)
    }
}

#[cfg(test)]
mod test{
    #[actix_rt::test]
    async fn load_xml() {
        crate::Osm4::load("./local/678240.osm4").await.unwrap();
    }
}