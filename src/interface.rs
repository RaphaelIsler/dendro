#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum ToFrontend{
    ProjectLoaded{project: crate::project::Project}
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub enum ToBackend {
    GetProject{id: crate::project::Id}
}

#[cfg(feature = "frontend")]
mod frontend {
    use super::*;
    use gloo_net::http::Request;

    impl ToBackend {
        pub async fn request(&self) -> Option<ToFrontend> {
            let fetched: ToFrontend = Request::post("/control")
                .json(self)
                .unwrap()
                .send()
                .await
                //            .body(serde_wasm_bindgen::to_value(self).unwrap()).unwrap().send().await
                .unwrap()
                .json()
                .await
                .unwrap();

            Some(fetched)
        }
    }
}
