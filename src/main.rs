mod models;

use std::convert::Infallible;
use std::collections::HashMap;
use std::sync::Arc;

use futures::future::BoxFuture;
use futures::future::FutureExt;

use warp::{Filter, Rejection};
use warp::http::StatusCode;
use warp::reject::not_found;
use warp::reply::{json, Json, Reply, with_status};
use models::Customer;

#[tokio::main]
async fn main() {

    let customer_db: HashMap<String, Customer> = vec![Customer::new_from_str("1", "Joe Bloggs"), Customer::new_from_str("2", "John Doe"), Customer::new_from_str("3", "Peter Parker")]
        .into_iter().map(|c| (c.id.clone(), c)).collect();

    let customer_repo = DummyCustomerRepository::new(customer_db);
    let repo: &dyn CustomerRepository = &customer_repo as &dyn CustomerRepository;

    let handler: &dyn CustomerHandler = repo as &dyn CustomerHandler;

    let hello = warp::path!("customers" / String)
        .and(warp::get())
        .and_then(handler.handle_get)
        .recover(handle_recover);

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn handle_recover(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        Ok(with_status("Not Found", StatusCode::NOT_FOUND))
    } else {
        Ok(with_status("Internal Error", StatusCode::INTERNAL_SERVER_ERROR))
    }
}

trait CustomerRepository {
    fn get_customer(&self, id: String) -> BoxFuture<'static, Option<Customer>>;
}

struct DummyCustomerRepository {
    db: Arc<HashMap<String, Customer>>,
}

impl DummyCustomerRepository {
    pub fn new(db: HashMap<String, Customer>) -> DummyCustomerRepository {
        DummyCustomerRepository { db: Arc::new(db) }
    }
}

impl CustomerRepository for DummyCustomerRepository {
    fn get_customer(&self, id: String) -> BoxFuture<'static, Option<Customer>> {
       let db = self.db.clone();
       async move { db.get(&id).map(|c| c.clone()) }.boxed()
    }
}

trait CustomerHandler {
    fn handle_get(&self, id: String) -> BoxFuture<'static, Result<Json, Rejection>>;
}

impl CustomerHandler for dyn CustomerRepository {
    fn handle_get(&self, id: String) -> BoxFuture<'static, Result<Json, Rejection>> {

        self.get_customer(id)
            .map( |maybe_customer| maybe_customer.map(|c| json(&c)).ok_or(not_found())).boxed()
    }
}