use ntex::web::types::Json;
use ntex::web::{self, middleware, App, HttpServer};
use serde::{Deserialize, Serialize};

use xml::reader::{EventReader, XmlEvent};

use std::{env, sync::Arc};

// use log::LogLevel;

use log::{debug, error, info, log_enabled, Level};

#[derive(Debug, Deserialize)]
struct Notification {
  id: u64,
  message: String,
  // Add more fields as needed
}

fn parse_xml(xml_string: &str) -> Result<(), Box<dyn std::error::Error>> {
  let parser = EventReader::from_str(xml_string);
  let mut current_element = String::new();

  for e in parser {
    match e {
      Ok(XmlEvent::StartElement { name, .. }) => {
        current_element = name.local_name.clone();
      }
      Ok(XmlEvent::Characters(text)) => {
        let trimmed_text = text.trim();
        if !trimmed_text.is_empty() {
          match current_element.as_str() {
            "ToUserName" => {
              println!("ToUserName: {}", trimmed_text);
            }
            "FromUserName" => {
              println!("FromUserName: {}", trimmed_text);
            }
            "CreateTime" => {
              println!("CreateTime: {}", trimmed_text);
            }
            "MsgType" => {
              println!("MsgType: {}", trimmed_text);
            }
            "Event" => {
              println!("Event: {}", trimmed_text);
            }
            "EventKey" => {
              println!("EventKey: {}", trimmed_text);
            }
            "Ticket" => {
              println!("Ticket: {}", trimmed_text);
            }
            _ => {}
          }
        }
      }
      Err(e) => {
        return Err(Box::new(e));
      }
      _ => {}
    }
  }

  Ok(())
}

#[derive(Debug, Serialize)]
struct ResponseData {
  status: String,
  // Add more fields as needed
}

async fn receive_notification(notification: Json<Notification>) -> impl web::Responder {
  // info!("This is an info log message");
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

  dotenvy::dotenv().ok();

  env::set_var("RUST_LOG", "ntex=info");
  env_logger::init();

  // env_logger::init_from_env(
  //   env_logger::Env::default().default_filter_or("info")
  // );

  debug!("this is a debug {}", "message");
  error!("this is printed by default");

  if log_enabled!(Level::Info) {
    let x = 3 * 4; // expensive computation
    info!("the answer was: {}", x);
  }

  let xml = r#"
  <xml>
      <ToUserName><![CDATA[toUser]]></ToUserName>
      <FromUserName><![CDATA[FromUser]]></FromUserName>
      <CreateTime>123456789</CreateTime>
      <MsgType><![CDATA[event]]></MsgType>
      <Event><![CDATA[subscribe]]></Event>
      <EventKey><![CDATA[qrscene_123123]]></EventKey>
      <Ticket><![CDATA[TICKET]]></Ticket>
  </xml>
"#;

  if let Err(e) = parse_xml(xml) {
    eprintln!("Error parsing XML: {}", e);
  }

  // Configure and start the ntex server
  ntex::web::server(|| {
    App::new().service(web::resource("/").route(web::post().to(receive_notification)))
  })
  .bind("127.0.0.1:19389")
  .unwrap()
  .run()
  .await
  .unwrap();
}

// use ntex::web;

// let app = web::App::new().service(
//     web::resource("/users/{userid}/{friend}")
//         .route(web::get().to(|| async { web::HttpResponse::Ok() }))
//         .route(web::head().to(|| async { web::HttpResponse::MethodNotAllowed() }))
// );
