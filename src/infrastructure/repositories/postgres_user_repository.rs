use std::sync::Arc;

use async_trait::async_trait;
use diesel::{result::Error, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use log::info;

use crate::{
    domain::{entities::user::User, repositories::user_repository::UserRepository}, 
    infrastructure::db::connection::{establish_connection, DBPool}, 
    persentation::handlers::user_handler::NewUser, schema::{self, users::{dsl::users, email}}
};


#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: DBPool
}

impl PostgresUserRepository {
    pub fn new() -> Self {
        let database_url = dotenv::var("DATABASE_URL").expect("DB URL......");
        info!("{database_url}");
        PostgresUserRepository {
            pool: establish_connection(&database_url)
        }
    }
}

impl Default for PostgresUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserRepository for Arc<PostgresUserRepository> {
    async fn find_by_email(&self, input_email: String) -> Option<User> {
        users.filter(email.eq(input_email))
            .first::<User>(& mut self.pool.get().unwrap())
            .optional()
            .expect("Error loading user")
    }

    async fn save(&self, user: &NewUser) -> Result<(),Error> {
        diesel::insert_into(schema::users::table)
            .values(user)
            .execute(& mut self.pool.get().unwrap())
            .unwrap();
        Ok(())
    }
}

