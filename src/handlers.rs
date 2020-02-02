use warp::{Rejection, Reply};
use std::convert::Infallible;
use futures::future::BoxFuture;
use futures::future::FutureExt;
use warp::http::StatusCode;
use warp::reply::{with_status, Json, json};
use crate::repos::CustomerRepository;
use warp::reject::not_found;

pub async fn handle_recover(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        Ok(with_status("Not Found", StatusCode::NOT_FOUND))
    } else {
        Ok(with_status("Internal Error", StatusCode::INTERNAL_SERVER_ERROR))
    }
}

pub fn handle(repo: &dyn CustomerRepository, id: String) -> BoxFuture<'static, Result<Json, Rejection>> {

    repo.get_customer(id)
        .map( |maybe_customer| maybe_customer.map(|c| json(&c)).ok_or(not_found())).boxed()
}