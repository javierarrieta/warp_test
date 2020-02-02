use std::collections::HashMap;
use std::sync::Arc;
use futures::future::BoxFuture;
use futures::future::FutureExt;
use crate::models::Customer;

pub trait CustomerRepository {
    fn get_customer(&self, id: String) -> BoxFuture<'static, Option<Customer>>;
}

#[derive(Clone)]
pub struct DummyCustomerRepository {
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
