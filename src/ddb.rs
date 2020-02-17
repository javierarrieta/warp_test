use std::collections::HashMap;
use std::sync::Arc;

use futures::future::{BoxFuture, Future, FutureExt};
use rusoto_core::RusotoError;
use rusoto_dynamodb::{AttributeValue, DynamoDb, GetItemError, GetItemInput, GetItemOutput};

use crate::models::Customer;
use crate::repos::CustomerRepository;
use std::pin::Pin;

pub struct CustomerDDBClient {
    client: Arc<dyn DynamoDb + Send + Sync>,
}

impl CustomerRepository for CustomerDDBClient {
    fn get_customer(&self, id: String) -> BoxFuture<'static, Option<Customer>>  {
        let request = GetItemInput::default();
        let client = self.client.clone();

        async move {
            let item = client.get_item(request).await.ok()?;
            get_item_to_customer(item)
        }.boxed()
    }
}

#[inline]
fn get_item_to_customer(get_item: GetItemOutput) -> Option<Customer> {
    get_item.item.and_then(parse_attributes_to_customer)
}

#[inline]
fn parse_attributes_to_customer(m: HashMap<String, AttributeValue>) -> Option<Customer> {
    unimplemented!()
}