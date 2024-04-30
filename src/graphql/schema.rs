use async_graphql::{Schema, EmptySubscription};
use super::query_root::QueryRoot;
use super::mutation_root::MutationRoot;

pub type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;