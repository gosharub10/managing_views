use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse};
use super::utils::return_state;
use crate::database::establish_connection;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::models::item::item::Item;
use crate::schema::to_do;


pub async fn delete(to_do_item : web::Json<ToDoItem>) -> HttpResponse{
    let title_reference = to_do_item.title.clone();

    let mut connection = establish_connection();
    let items = to_do::table
    .filter(to_do::columns::title.eq(title_reference.as_str()))
    .order(to_do::columns::id.asc())
    .load::<Item>(&mut connection)
    .unwrap();

    let _ = diesel::delete(&items[0]).execute(&mut connection);

    return HttpResponse::Ok().json(return_state())
}