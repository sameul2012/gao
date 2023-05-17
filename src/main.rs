use ntex::web::types::Json;
use ntex::web::{self, middleware, App, HttpServer};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Notification {
  id: u64,
  message: String,
  // Add more fields as needed
}

#[derive(Debug, Serialize)]
struct ResponseData {
  status: String,
  // Add more fields as needed
}

async fn receive_notification(notification: Json<Notification>) -> impl web::Responder {
  // Extract the important information from the notification
  let notification_id = notification.id;
  let notification_message = &notification.message;

  // Process the notification and generate a response
  let response_data = ResponseData {
    status: "success".to_owned(),
    // Add more fields and data as needed
  };

  // Return a JSON response
  web::HttpResponse::Ok().json(&response_data)
}

// can also use #[actix_web::main]

#[ntex::main]
async fn main() {
  println!("Hello, world!");

  // Configure and start the ntex server
  ntex::web::server(|| {
    App::new().service(web::resource("/notify").route(web::post().to(receive_notification)))
  })
  .bind("127.0.0.1:19389")
  .unwrap()
  .run()
  .await
  .unwrap();
}
