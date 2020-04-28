use crate::db::ConnectionPool;
use diesel::prelude::*;
use juniper::{FieldResult, RootNode};

#[derive(Queryable)]
pub struct Game {
    id: i32,
    name: String,
    image: String,
}

#[derive(Queryable)]
pub struct Server {
    id: i32,
    name: String,
    game_id: i32,
    status: String,
}

#[juniper::object(
    Context = Context,
)]
impl Game {
    pub fn id(&self) -> &i32 {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn image(&self) -> &str {
        &self.image
    }
}

#[juniper::object(
    Context = Context,
)]
impl Server {
    pub fn id(&self) -> &i32 {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn game(&self, context: &Context) -> FieldResult<Game> {
        context.get_game(self.game_id)
    }

    pub fn status(&self) -> &str {
        &self.status
    }
}

pub struct Context {
    pub pool: ConnectionPool,
}

impl juniper::Context for Context {}

impl Context {
    pub fn get_game(&self, game_id: i32) -> FieldResult<Game> {
        use crate::schema::games::dsl::*;
        let connection = self.pool.get().unwrap();
        Ok(games.filter(id.eq(game_id)).first(&connection)?)
    }

    pub fn get_server(&self, server_id: i32) -> FieldResult<Server> {
        use crate::schema::servers::dsl::*;
        let connection = self.pool.get().unwrap();
        Ok(servers.filter(id.eq(server_id)).first(&connection)?)
    }
}

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn games(context: &Context) -> FieldResult<Vec<Game>> {
        use crate::schema::games::dsl::*;
        let connection = context.pool.get().unwrap();
        Ok(games.load(&connection)?)
    }

    fn servers(context: &Context) -> FieldResult<Vec<Server>> {
        use crate::schema::servers::dsl::*;
        let connection = context.pool.get().unwrap();
        Ok(servers.load(&connection)?)
    }
}

pub struct Mutations;

#[juniper::object(
    Context = Context,
)]
impl Mutations {
    fn start_stop_server(context: &Context, server_id: i32) -> FieldResult<Server> {
        let server = context.get_server(server_id)?;
        println!("Current Status: {}", server.status);
        Ok(server)
    }
}

pub type Schema = RootNode<'static, Query, Mutations>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutations {})
}