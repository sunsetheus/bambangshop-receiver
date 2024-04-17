use rocket::http::Status;
use rocket::log;
use rocket::serde::json::to_string;
use rocket::tokio;
use std::thread;

use crate::model::notification::Notification;
use crate::model::subscriber::SubscriberRequest;
use crate::repository::notification::NotificationRepository;
use bambangshop_receiver::{compose_error_response, Result, APP_CONFIG, REQWEST_CLIENT};

pub struct NotificationService;

impl NotificationService {
    #[tokio::main]
    async fn subscribe_request(product_type: String) -> Result<SubscriberRequest> {
        let product_type_upper = product_type.to_uppercase();
        let product_type_str = product_type_upper.as_str();
        let notification_reciever_url = format!("{}/recieve", APP_CONFIG.get_instance_root_url());
        let payload = SubscriberRequest {
            name: APP_CONFIG.get_instance_name().to_string(),
            url: notification_reciever_url,
        };
        let request_url = format!(
            "{}/notification/subscribe/{}",
            APP_CONFIG.get_instance_root_url(),
            product_type_str
        );
        let request = REQWEST_CLIENT
            .post(request_url.clone())
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(to_string(&payload).unwrap())
            .send()
            .await;
        log::warn_!("Sent subscribe request to {}", request_url);
        return match request {
            Ok(f) => match f.json::<SubscriberRequest>().await {
                Ok(x) => Ok(x),
                Err(y) => Err(compose_error_response(Status::NotAcceptable, y.to_string())),
            },
            Err(e) => Err(compose_error_response(Status::NotFound, e.to_string())),
        };
    }

    pub fn subscribe(product_type: &str) -> Result<SubscriberRequest> {
        let product_type_clone = String::from(product_type);
        return thread::spawn(move || Self::subscribe_request(product_type_clone))
            .join()
            .unwrap();
    }

    #[tokio::main]
    async fn unsubscribe_request(product_type: String) -> Result<SubscriberRequest> {
        let product_type_upper = product_type.to_uppercase();
        let product_type_str = product_type_upper.as_str();
        let notification_reciever_url = format!("{}/recieve", APP_CONFIG.get_instance_root_url());

        let request_url = format!(
            "{}/notification/unsubscribe/{}?url={}",
            APP_CONFIG.get_instance_root_url(),
            product_type_str,
            notification_reciever_url
        );
        let request = REQWEST_CLIENT
            .post(request_url.clone())
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .send()
            .await;
        log::warn_!("Sent unsubscribe request to: {}", request_url);

        return match request {
            Ok(f) => match f.json::<SubscriberRequest>().await {
                Ok(x) => Ok(x),
                Err(_) => Err(compose_error_response(
                    Status::NotAcceptable,
                    String::from("Already unsubscribe from topic"),
                )),
            },
            Err(e) => Err(compose_error_response(Status::NotFound, e.to_string())),
        };
    }

    pub fn unsubscribe(product_type: &str) -> Result<SubscriberRequest> {
        let product_type_clone = String::from(product_type);
        return thread::spawn(move || Self::unsubscribe_request(product_type_clone))
            .join()
            .unwrap();
    }

    pub fn recieve_notification(payload: Notification) -> Result<Notification> {
        let subscriber_result = NotificationRepository::add(payload.clone());
        return Ok(subscriber_result);
    }

    pub fn list_messages() -> Result<Vec<String>> {
        return Ok(NotificationRepository::list_all_as_string());
    }
}