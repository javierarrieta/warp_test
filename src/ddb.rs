use std::collections::HashMap;
use std::sync::Arc;

use futures::future::{BoxFuture, Future, FutureExt};
use rusoto_core::RusotoError;
use rusoto_dynamodb::{AttributeValue, DynamoDb, GetItemError, GetItemInput, GetItemOutput};

use crate::models::Customer;
use crate::repos::CustomerRepository;
use std::pin::Pin;

pub struct CustomerDDBClient {
    client: Arc<dyn DynamoDb>,
}

impl CustomerRepository for CustomerDDBClient {
    fn get_customer(&self, id: String) -> BoxFuture<'static, Option<Customer>>  {
        let request = GetItemInput::default();
        self.client.clone().get_item(request).map(|r|
            r.ok().and_then(get_item_to_customer).map(|c| c.clone() )
        ).boxed()
    }
}

fn get_item_to_customer(get_item: GetItemOutput) -> Option<Customer> {
    unimplemented!()
}

fn parse_attributes_to_customer(m: HashMap<String, AttributeValue>) -> Customer {
    unimplemented!()
}