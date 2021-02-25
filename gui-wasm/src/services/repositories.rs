use yew::{Callback, services::fetch::{FetchTask}};

use serde::{Deserialize, Serialize};

use crate::services::api::Api;
use crate::error::ApiError;

#[derive(Serialize, Default, Deserialize, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct RepositoriesResponse {
    pub repositories: Vec<String>,
}

pub struct RepositoriesSvc {
    svc: Api,
}

impl RepositoriesSvc {
    pub fn new() -> Self {
        Self { svc: Api::new() }
    }

    pub fn fetch(
        &mut self,
        callback: Callback<Result<RepositoriesResponse, ApiError>>,
    ) -> FetchTask {
        self.svc
            .get::<RepositoriesResponse>(String::from("/v2/_catalog"), callback)
    }

    // pub fn fetch_by_limit(
    //     &mut self,
    //     limit: u32,
    //     callback: Callback<Result<RepositoriesResponse, ApiError>>,
    // ) -> FetchTask {
    //     self.svc
    //         .get::<RepositoriesResponse>(format!("v2/_catalog?{}", limit), callback)
    // }

    // pub fn fetch_by_limit_and_last_repo(
    //     &mut self,
    //     limit: u32,
    //     last_repo: String,
    //     callback: Callback<Result<RepositoriesResponse, ApiError>>,
    // ) -> FetchTask {
    //     self.svc
    //         .get::<RepositoriesResponse>(format!("v2/_catalog?{}&{}", limit, last_repo), callback)
    // }
}
