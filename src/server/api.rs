// use async_graphql::{ MergedObject, Schema, EmptySubscription, http::{ playground_source, GraphQLPlaygroundConfig } };
// use poem::{ handler, IntoResponse, web::Html };

// use super::api_auth::{ AuthQuery, AuthMutation };

// // Get in queries
// // use crate::database::models::ModelsQuery;

// #[derive( Default )]
// pub struct BaseQuery;
// #[derive( Default )]
// pub struct BaseMutation;

// // #[Object]
// // impl BaseQuery {
// //     /// Returns the sum of a and b
// //     async fn add( &self, a: i32, b: i32 ) -> i32 {
// //         a + b
// //     }
// // }

// #[derive(MergedObject, Default)]
// pub struct Query( /*BaseQuery,*/ AuthQuery );

// // #[Object]
// // impl BaseMutation {
// //     /// Returns the result of subtracting 'b' from 'a'
// //     async fn subtract( &self, a: i32, b: i32 ) -> i32 {
// //         a - b
// //     }
// // }

// #[derive(MergedObject, Default)]
// pub struct Mut( /*BaseMutation,*/ AuthMutation );

// pub fn start_schema() -> async_graphql::SchemaBuilder<Query, Mut, EmptySubscription> {
//     return Schema::build( Query::default(), Mut::default(), EmptySubscription );
// }

// #[handler]
// pub fn graphql_playground() -> impl IntoResponse {
//     Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
// }