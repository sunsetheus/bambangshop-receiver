use rocket::serde::json::Json;

use bambangshop_receiver::Result;
use crate::model::notification::{self, Notification};
use crate::model::subscriber::SubscriberRequest;
use crate::service::notification::NotificationService;

#[get("/subscribe/<product_type>")]
pub fn subscribe(product_type: &str)->Result<Json<SubscriberRequest>>{
    return match NotificationService::subscribe(product_type){
        Ok(f) => Ok(Json::from(f)),
        Err(e) => Err(e)
    }
}

#[get("/unsubscribe/<product_type>")]
pub fn unsubscribe(product_type: &str)-> Result<Json<SubscriberRequest>>{
    return match NotificationService::unsubscribe(product_type){
        Ok(f) => Ok(Json::from(f)),
        Err(e)=> Err(e)
    }
}

#[post("/recieve", data = "<notification>")]
pub fn recieve(notification: Json<Notification>) -> Result<Json<Notification>> {
    return match NotificationService::recieve_notification(notification.into_inner()) {
        Ok(f) => Ok(Json::from(f)),
        Err(e) => Err(e),
    };
}

#[get("/")]
pub fn list() -> Result<Json<Vec<String>>> {
    return match NotificationService::list_messages() {
        Ok(f) => Ok(Json::from(f)),
        Err(e) => Err(e),
    };
}