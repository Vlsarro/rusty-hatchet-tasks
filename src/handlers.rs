use cot::Template;
use cot::html::Html;
use cot::request::extractors::Path;
use cot::request::extractors::StaticFiles;

#[derive(Debug, Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    static_files: StaticFiles,
}

pub async fn index(static_files: StaticFiles) -> cot::Result<Html> {
    let index_template = IndexTemplate { static_files };
    let rendered = index_template.render()?;

    Ok(Html::new(rendered))
}

pub async fn run_task() -> cot::Result<Html> {
    Ok(Html::new("run task"))
}

pub async fn get_task_status(Path(task_id): Path<String>) -> cot::Result<Html> {
    Ok(Html::new(format!("got task {}", task_id)))
}
