use async_graphql::{Object, Result};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn example_query(&self) -> Result<String>  {
        Ok("Hello, world!".to_string())
    }
}