use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha512};
use sqlx::PgPool;

type Pool = PgPool;

#[derive(sqlx::FromRow, Debug)]
#[allow(unused)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub session_id: Option<String>,
}

/// Handles all the connections for the `users` table
pub mod account {
    use sqlx::Error;

    use super::{get_session_id, Hasher, Pool, User};

    pub async fn register(conn: &Pool, username: &str, password: &str) -> Result<User, Error> {
        sqlx::query(include_str!("sql/register.sql"))
            .bind(username)
            .bind(password.to_string().to_sha512())
            .execute(conn)
            .await?;

        // We can unwrap here because we have just created the user
        Ok(login(conn, username, password).await?.unwrap())
    }

    pub async fn login(conn: &Pool, username: &str, password: &str) -> Result<Option<User>, Error> {
        let password = password.to_string().to_sha512();

        let mut user = sqlx::query_as::<_, User>(include_str!("sql/login.sql"))
            .bind(&username)
            .bind(&password)
            .fetch_optional(conn)
            .await?;

        // We update the session id if it does not exist. The indentation level is
        // criminal, i'm sorry
        if let Some(user) = &mut user {
            if user.session_id.is_none() {
                user.session_id = Some(
                    update_session_id(conn, username, &password)
                        .await?
                        .unwrap() // we just checked if the user exists so we can unwrap
                        .session_id
                        .unwrap(), // guaranteed to have set a new session id so we can unwrap
                );
            }
        }

        Ok(user)
    }

    // -- Utility functions --
    async fn update_session_id(
        conn: &Pool,
        username: &str,
        password: &str,
    ) -> Result<Option<User>, Error> {
        // We update the session id
        sqlx::query(include_str!("sql/update_session_id.sql"))
            .bind(get_session_id())
            .bind(username)
            .bind(password)
            .execute(conn)
            .await?;

        // And refetch the user
        sqlx::query_as::<_, User>(include_str!("sql/login.sql"))
            .bind(&username)
            .bind(&password)
            .fetch_optional(conn)
            .await
    }
}

fn get_session_id() -> String {
    let unix_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    unix_timestamp.to_string().to_sha512()
}

// -- Utility functions --
pub async fn get_connection() -> Pool {
    PgPool::connect(env!("DATABASE_URL")).await.unwrap()
}

pub trait Hasher {
    fn to_sha512(&self) -> String;
}

impl Hasher for String {
    fn to_sha512(&self) -> String {
        let mut hasher = Sha512::new();
        hasher.update(self.as_bytes());

        let hash_result = hasher.finalize();
        hash_result
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
    }
}
