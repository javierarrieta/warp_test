mod models;
mod repos;
mod handlers;
mod ddb;

use warp::Filter;
use handlers::{handle, handle_recover};
use rusoto_dynamodb::DynamoDbClient;
use rusoto_core::Region;
use crate::ddb::CustomerDDBClient;
use std::sync::Arc;

#[tokio::main]
async fn main() {

//    let customer_db: HashMap<String, Customer> = vec![Customer::new_from_str("1", "Joe Bloggs"), Customer::new_from_str("2", "John Doe"), Customer::new_from_str("3", "Peter Parker")]
//        .into_iter().map(|c| (c.id.clone(), c)).collect();

    let ddb_client: DynamoDbClient = DynamoDbClient::new(Region::EuCentral1);

    let customer_repo = CustomerDDBClient { client: Arc::new(ddb_client), table_name: "javi_customer".to_owned()  };

    let hello = warp::path!("customers" / String)
        .and(warp::get())
        .and_then(move |id| handle(&customer_repo, id))
        .recover(handle_recover);

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
