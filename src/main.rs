// use crate::web::HttpRequest;
mod errors;

use ntex::util::Bytes;
use ntex::web::test::read_body;
use ntex::web::types::Json;
use ntex::web::types::Payload;
// use ntex::web::Bytes;
use ntex::web::FromRequest;
use ntex::web::HttpRequest;
use ntex::web::HttpResponse;
use ntex::web::{self, types};

use ntex::web::{middleware, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use serde::{Deserialize, Serialize};

use xml::reader::{EventReader, XmlEvent};

use std::{env, sync::Arc};

use serde_xml_rs::from_str;

// use log::LogLevel;
use log::info;
use log::{debug, error, log_enabled, Level};

mod neolog;
use neolog::mylog;

mod weixin;
use weixin::notify;

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

#[derive(Debug, Deserialize)]
struct XmlData {
  #[serde(rename = "ToUserName")]
  to_user_name: String,
  #[serde(rename = "FromUserName")]
  from_user_name: String,
  #[serde(rename = "CreateTime")]
  create_time: u64,
  #[serde(rename = "MsgType")]
  msg_type: String,
  #[serde(rename = "Event")]
  event: String,
  #[serde(rename = "EventKey")]
  event_key: String,
  #[serde(rename = "Ticket")]
  ticket: String,
}

// async fn handle_post(json: web::types::Json<JsonData>) -> web::types::HttpResponse {
//   // 使用 json 中的字段值进行后续操作
//   println!("Field 1: {}", json.field1);
//   println!("Field 2: {}", json.field2);
//   // 返回响应
//   web::types::HttpResponse::Ok().body("JSON data received")
// }

async fn receive_notification(notification: Json<Notification>) -> impl web::Responder {
  // info!("This is an info log message");
  info!("this is an info {}", "message");
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

//  mut payload: web::PayloadStream
//  web::types::HttpResponse  payload: web::types::Payload
async fn handle_post(payload: web::types::Payload) -> ntex::web::HttpResponse {
  // // 从请求中获取请求体的字节流
  // let body_bytes = web::types::body::Bytes::from_request(&request)
  //   .await
  //   .unwrap();
  // let v1 = &HttpRequest
  // let v2 = &mut ntex::http::Payload;
  // let body_bytes = web::types::Payload::from_request(&HttpRequest, &payload)
  //   .await
  //   .unwrap();

  let str1 = String::from("handle_post run");
  info!("this is an info {:#?}", payload);
  mylog::bee(&str1);
  // Read the request payload into a byte buffer
  // let body_bytes = web::types::Bytes::from_request(&payload).await.unwrap();

  // Read the request payload into a byte buffer
  // let body_bytes = web::types::Bytes::from_request(&payload).await.unwrap();

  // Read the request payload into a byte buffer
  // let body_bytes = ntex::web::test::read_body(payload).await.unwrap();

  // let mut body = web::Bytes::new();
  // while let Some(chunk) = payload.next().await {
  //   let chunk = chunk.unwrap();
  //   body.extend_from_slice(&chunk);
  // }

  // let body = web::types::take_payload(payload).await;
  // let body_bytes = body.as_ref().unwrap().to_vec();

  // // 将字节流转换为字符串
  // let body_string = String::from_utf8_lossy(&body_bytes).to_string();

  // Convert the byte buffer to a string
  // let body_string = String::from_utf8_lossy(&body_bytes).to_string();

  // // 解析 XML 字符串为相应的结构体或使用自定义的 XML 解析库进行处理
  // // 在这里，我们使用 serde_xml_rs 进行示例解析
  // let xml_data: serde_xml_rs::de::Result<XmlData> = serde_xml_rs::from_str(&body_string);

  // Parse the XML string into the desired struct or use a custom XML parsing library
  // Here, we use serde_xml_rs for demonstration
  // let xml_data: Result<XmlData, serde_xml_rs::Error> = serde_xml_rs::from_str(&body_string);

  // match xml_data {
  //   Ok(data) => {
  //     // 成功解析 XML 数据，可以访问 data 中的字段值进行后续处理
  //     println!("{:?}", data);
  //     // 处理 XML 数据
  //     // ...
  //   }
  //   Err(err) => {
  //     // 解析 XML 数据失败
  //     println!("Failed to parse XML: {}", err);
  //     // 处理解析错误
  //     // ...
  //   }
  // }

  // 返回响应
  // web::types::HttpResponse::Ok().finish()
  ntex::web::HttpResponse::Ok().finish()
}

#[derive(Debug, Clone)]
pub struct AppState {
  pub db_pool: Pool<Postgres>,
}

// #[ntex::main]
// async fn main() -> std::io::Result<()> {

// }

#[ntex::main]
async fn main() {
  println!("Hello, world!");

  dotenvy::dotenv().ok();
  env::set_var("RUST_LOG", "ntex=info");
  env_logger::init();

  let str1 = String::from("main sta");
  mylog::bee(&str1);

  // env_logger::init_from_env(
  //   env_logger::Env::default().default_filter_or("info")
  // );
  info!("this is an info {}", "message");
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

  let db_url = env::var("DATABASE_URL").expect("Pls set `DATABASE_URL` in env or env var");

  // State
  let app_state = Arc::new(AppState {
    db_pool: PgPoolOptions::new()
      .max_connections(10)
      .connect(&db_url)
      .await
      .unwrap(),
  });

  // let app = web::App::new().service(
  //   web::scope("/{project_id}")
  //     .service(web::resource("/path1").to(|| async { web::HttpResponse::Ok() }))
  //     .service(web::resource("/path2").to(|| async { web::HttpResponse::Ok() }))
  //     .service(web::resource("/path3").to(|| async { web::HttpResponse::MethodNotAllowed() })),
  // );

  HttpServer::new(move || {
    App::new()
      .wrap(middleware::Logger::default())
      .configure(|cfg| route(Arc::clone(&app_state), cfg))
  })
  .bind("0.0.0.0:9389")
  .unwrap()
  .run()
  .await
  .unwrap();

  // HttpServer::new(|| App::new()

  //       .service(web::resource("/").to(|| async { HttpResponse::Ok() })))
  //   .bind("127.0.0.1:59090")?
  //   .run()
  //   .await

  // // Configure and start the ntex server
  // ntex::web::server(|| App::new().service(web::resource("/").route(web::post().to(handle_post))))
  //   .bind("127.0.0.1:19389")
  //   .unwrap()
  //   .run()
  //   .await
  //   .unwrap();
}

fn route(_state: Arc<AppState>, cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/").route("", web::post().to(notify::new_notify)));
}

// fn route(_state: Arc<AppState>, cfg: &mut web::ServiceConfig) {
//   cfg
//     .service(
//       web::scope("/article")
//         .route("/{id}", web::get().to(view::get_article))
//         .route("", web::post().to(new::new_article))
//         .route("", web::put().to(edit::edit_article))
//         .route("/{id}", web::delete().to(delete::delete_article))
//         .route("/search/{keyword}", web::get().to(search::search_article)),
//     )
//     .service(web::scope("/articles").route("", web::get().to(view::get_articles_preview)))
//     .service(web::scope("/user").route("/login", web::post().to(login::github_login)));
// }

// use ntex::web;

// let app = web::App::new().service(
//     web::resource("/users/{userid}/{friend}")
//         .route(web::get().to(|| async { web::HttpResponse::Ok() }))
//         .route(web::head().to(|| async { web::HttpResponse::MethodNotAllowed() }))
// );
