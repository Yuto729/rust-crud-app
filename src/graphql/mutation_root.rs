use async_graphql::{Object, Result};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn example_mutation(&self, input: String) -> Result<String> {
        Ok(format!("Received: {}", input))
    }
}