use sqlx::PgPool;

type Pool = PgPool;

#[derive(sqlx::FromRow, Debug)]
#[allow(unused)]
pub struct User {
    id: i32,
    username: String,
    password: String,
    session_id: Option<String>,
}

pub async fn get_connection() -> Pool {
    PgPool::connect(env!("DATABASE_URL")).await.unwrap()
}

/// Handles all the connections for the `users` table
pub mod account {
    use sqlx::Error;

    use super::{Pool, User};

    pub async fn register(conn: &Pool, username: &str, password: &str) -> Result<User, Error> {
        sqlx::query(include_str!("sql/register.sql"))
            .bind(username)
            .bind(password)
            .execute(conn)
            .await?;

        // We can unwrap here because we have just created the user
        Ok(login(conn, username, password).await?.unwrap())
    }

    pub async fn login(conn: &Pool, username: &str, password: &str) -> Result<Option<User>, Error> {
        sqlx::query_as::<_, User>(include_str!("sql/login.sql"))
            .bind(username)
            .bind(password)
            .fetch_optional(conn)
            .await
    }
}
