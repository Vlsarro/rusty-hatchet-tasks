use cot::Template;
use cot::html::Html;
use cot::request::Request;
use cot::request::extractors::Path;
use cot::request::extractors::StaticFiles;
use cot::response::Response;
use cot::reverse_redirect;

use cot::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    static_files: StaticFiles,
}

pub async fn index(request: Request) -> cot::Result<Response> {
    Ok(reverse_redirect!(request, "tasks")?)
}

pub async fn tasks(static_files: StaticFiles) -> cot::Result<Html> {
    let index_template = IndexTemplate { static_files };
    let rendered = index_template.render()?;

    Ok(Html::new(rendered))
}

#[derive(Debug, Clone, Deserialize, schemars::JsonSchema)]
pub struct CreateTaskRequest {
    r#type: String,
}

#[derive(Debug, Clone, Serialize, schemars::JsonSchema)]
pub struct CreateTaskResponse {
    task_id: String,
}

#[derive(Debug, Clone, Serialize, schemars::JsonSchema)]
pub struct GetTaskResponse {
    task_id: String,
    task_status: String,
    task_result: String,
}

const TASK_ID: &str = "324234234";

pub async fn create_task(Json(_): Json<CreateTaskRequest>) -> Json<CreateTaskResponse> {
    let response = CreateTaskResponse {
        task_id: String::from(TASK_ID),
    };
    Json(response)
}

pub async fn get_task(Path(_): Path<String>) -> Json<GetTaskResponse> {
    let response = GetTaskResponse {
        task_id: String::from(TASK_ID),
        task_result: String::from("123"),
        task_status: String::from("SUCCESS"),
    };
    Json(response)
}
