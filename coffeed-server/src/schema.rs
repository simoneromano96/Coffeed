use actix_web::{web, Error, HttpResponse};
use futures::Future;
use juniper::graphiql::graphiql_source;
use juniper::http::{playground::playground_source, GraphQLRequest};
use juniper::{Executor, FieldResult};
use std::sync::Arc;
use uuid::Uuid;

use juniper_from_schema::graphql_schema_from_file;
graphql_schema_from_file!("src/schema.graphql");

pub struct Context {
    // wire up db here db_con: DbCon,
}
impl juniper::Context for Context {}

pub struct Query;
pub struct Mutation;

pub struct Coffee {
    id: Uuid,
    name: String,
    price: f64,
    description: Option<String>,
}

impl CoffeeFields for Coffee {
    fn field_id(&self, _: &Executor<'_, Context>) -> FieldResult<juniper::ID> {
        Ok(juniper::ID::new(self.id.to_string()))
    }
    fn field_name(&self, _: &Executor<'_, Context>) -> FieldResult<&String> {
        Ok(&self.name)
    }
    fn field_price(&self, _: &Executor<'_, Context>) -> FieldResult<&f64> {
        Ok(&self.price)
    }
    fn field_description(&self, _: &Executor<'_, Context>) -> FieldResult<&Option<String>> {
        Ok(&self.description)
    }
}

/*
pub struct CoffeeInput {
    name: String,
    price: f64,
    description: String,
}
*/

// Query resolvers
impl QueryFields for Query {
    fn field_coffees(
        &self,
        _context: &Executor<'_, Context>,
        _parent: &juniper_from_schema::QueryTrail<Coffee, juniper_from_schema::Walked>,
    ) -> FieldResult<Vec<Coffee>> {
        let mut result = Vec::new();

        let coffee = Coffee {
            id: Uuid::new_v4(),
            name: String::from("My coffee"),
            price: 0.5,
            description: Some(String::from("Hello")),
        };

        result.push(coffee);
        Ok(result)
    }

    fn field_coffee(
        &self,
        _context: &juniper::Executor<'_, Context>,
        _parent: &juniper_from_schema::QueryTrail<Coffee, juniper_from_schema::Walked>,
        _id: juniper::ID,
    ) -> FieldResult<Option<Coffee>> {
        let result = Coffee {
            id: Uuid::new_v4(),
            name: String::from("My coffee"),
            price: 0.5,
            description: Some(String::from("Hello")),
        };
        Ok(Some(result))
    }
}

// Mutation resolvers
impl MutationFields for Mutation {
    fn field_create_coffee(
        &self,
        _executor: &Executor<'_, Context>,
        _trail: &QueryTrail<'_, Coffee, Walked>,
        name: String,
        price: f64,
        description: Option<String>,
    ) -> FieldResult<Option<Coffee>> {
        let result = Coffee {
            id: Uuid::new_v4(),
            name,
            price,
            description,
        };
        Ok(Some(result))
    }
}

fn playground() -> HttpResponse {
    let html = playground_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:8080/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn graphql(
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
    // db_pool: web::Data<DbPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let ctx = Context {
        // db_con: db_pool.get().unwrap(),
    };

    web::block(move || {
        let res = data.execute(&schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

pub fn register(config: &mut web::ServiceConfig) {
    let schema = std::sync::Arc::new(Schema::new(Query, Mutation));

    config
        .data(schema)
        .route("/graphql", web::post().to_async(graphql))
        .route("/playground", web::get().to(playground))
        .route("/graphiql", web::get().to(graphiql));
}
