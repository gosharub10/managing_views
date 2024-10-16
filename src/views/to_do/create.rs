use crate::diesel;
use diesel::prelude::*;
use actix_web::HttpRequest;
use actix_web::Responder;
use crate::database::establish_connection;
use crate::models::item::new_item::NewItem;
use crate::models::item::item::Item;
use crate::schema::to_do;
use super::utils::return_state;

pub async fn create(req: HttpRequest) -> impl Responder{
    let title: String = req.match_info().get("title").unwrap().to_string();
    let title_reference: String = title.clone();
    let mut connection = establish_connection();
    let items = to_do::table.filter(to_do::columns::title.eq(title_reference.as_str()))
                        .order(to_do::columns::id.asc())
                        .load::<Item>(&mut connection).unwrap();

    if items.len() == 0{
        let new_post = NewItem::new(title);
        let _ = diesel::insert_into(to_do::table)
                .values(new_post)
                .execute(&mut connection);
    }
    return return_state();
}