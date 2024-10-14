use actix_web::web::{self};
mod create;
use super::path::Path;
mod get;
mod utils;
mod edit;
mod delete;

pub fn item_factory(app: &mut web::ServiceConfig){
    let base_path: Path = Path{prefix: String::from("/item")};
    let _app = app.route(&base_path.define(String::from("/create/{title}")),
              web::post().to(create::create))
                    .route(&base_path.define(String::from("/get")),
              web::get().to(get::get))
                    .route(&base_path.define(String::from("/edit")),
              web::put().to(edit::edit))
                    .route(&base_path.define(String::from("/delete")),
              web::post().to(delete::delete));
}