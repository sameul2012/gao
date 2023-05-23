use std::sync::Arc;

use crate::errors::CustomError;

// use ntex::{
//     http::error,
//     web::{
//         self,
//         types::{Json, State},
//         HttpResponse, Responder,
//     },
// };
use ntex::web::{
  // self,
  types::{Json, State},
  HttpResponse,
  Responder,
};

// use crate::{errors::CustomError, models::article::Article, AppState};
use crate::AppState;

pub async fn new_notify(state: State<Arc<AppState>>) -> Result<impl Responder, CustomError> {
  // let db_pool = &state.db_pool;
  // sqlx::query!(
  //     "INSERT INTO articles (title, content) VALUES ($1, $2)",
  //     article.title,
  //     article.content
  // )
  // .execute(db_pool)
  // .await?;

  Ok(HttpResponse::Created().body("rcv notify success"))
}
