use std::collections::HashMap;
use std::sync::Arc;

use futures::future::{BoxFuture, FutureExt};
use rusoto_dynamodb::{AttributeValue, DynamoDb, GetItemInput, GetItemOutput};

use crate::models::Customer;
use crate::repos::CustomerRepository;

#[derive(Clone)]
pub struct CustomerDDBClient {
    pub client: Arc<dyn DynamoDb + Send + Sync>,
    pub table_name: String,
}

impl CustomerRepository for CustomerDDBClient {
    fn get_customer(&self, id: String) -> BoxFuture<'static, Option<Customer>>  {
        let mut request = GetItemInput::default();

        let mut key_attr_value = AttributeValue::default();
        key_attr_value.n = Some(id);
        let mut key: HashMap<String, AttributeValue> = HashMap::new();
        key.insert("customer_id".to_owned(), key_attr_value);

        request.table_name = self.table_name.clone();
        request.key = key;

        let client = self.client.clone();

        async move {
            let item = client.get_item(request).await.ok()?;
            get_item_to_customer(&item)
        }.boxed()
    }
}

#[inline]
fn get_item_to_customer(get_item: &GetItemOutput) -> Option<Customer> {
    get_item.item.as_ref().and_then( |m| parse_attributes_to_customer(&m))
}

#[inline]
fn parse_attributes_to_customer(m: &HashMap<String, AttributeValue>) -> Option<Customer> {
    let id = m.get("customer_id").and_then(|v| v.n.clone())?;
    let name = m.get("name").and_then(|v| v.n.clone())?;

    Some(Customer::new(id, name))
}