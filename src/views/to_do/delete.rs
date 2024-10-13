use actix_web::{web, HttpResponse};

use super::utils::return_state;
use crate::state::read_file;

use crate::to_do::to_do_factory;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::processes::process_input;


pub async fn delete(to_do_item : web::Json<ToDoItem>) -> HttpResponse{
    let state = read_file(String::from("./state.json"));
    let title = to_do_item.title.clone();
    let status = to_do_item.status.clone();

    match to_do_factory(&status, &title) {
        Err(_item) => return  HttpResponse::BadRequest().json(
            format!("{} not accepted", _item)),
        Ok(item) => process_input(item, String::from("delete"), &state)
    }

    return HttpResponse::Ok().json(return_state())
}