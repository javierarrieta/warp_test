mod models;
mod repos;
mod handlers;

use std::collections::HashMap;

use warp::Filter;
use models::Customer;
use repos::DummyCustomerRepository;
use handlers::{handle, handle_recover};

#[tokio::main]
async fn main() {

    let customer_db: HashMap<String, Customer> = vec![Customer::new_from_str("1", "Joe Bloggs"), Customer::new_from_str("2", "John Doe"), Customer::new_from_str("3", "Peter Parker")]
        .into_iter().map(|c| (c.id.clone(), c)).collect();

    let customer_repo = DummyCustomerRepository::new(customer_db);

    let hello = warp::path!("customers" / String)
        .and(warp::get())
        .and_then(move |id| handle(&customer_repo, id))
        .recover(handle_recover);

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
