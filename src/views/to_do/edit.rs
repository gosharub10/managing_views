use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};
use super::utils::return_state;
use crate::database::establish_connection;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::schema::to_do;

pub async fn edit(to_do_item: web::Json<ToDoItem>) -> HttpResponse {

    let title_reference: &String = &to_do_item.title.clone();
    let mut connection = establish_connection();
    let result = to_do::table.filter(to_do::columns::title.eq(title_reference));

    let _ = diesel::update(result).set(to_do::columns::status.eq("done")).execute(&mut connection);

    return HttpResponse::Ok().json(return_state());
}