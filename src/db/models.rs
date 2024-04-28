
use std::sync::Arc;

use sqlx::PgPool;
use  anyhow::Result;


#[derive(sqlx::FromRow)]
struct User {
    id: i64,
    username: String,
    email: String,
    password_hash: String,
}

impl User {
    fn new(id: i64, username: String, email: String, password: String) -> User {
        let password_hash = hash_password(&password);
        User { id, username, email, password_hash }
    }

    fn authenticate(&self, password: &str) -> bool {
        // Compare the provided password with the stored hash
        verify_password(password, &self.password_hash)
    }

    fn display(&self) {
        println!("User ID: {}", self.id);
        println!("Username: {}", self.username);
        println!("Email: {}", self.email);
    }
}

// Dummy functions to hash and verify passwords
fn hash_password(password: &str) -> String {
    // In a real application, use a secure hashing algorithm like bcrypt
    format!("hash({})", password)
}

fn verify_password(password: &str, hash: &str) -> bool {
    // In a real application, use a secure method to verify passwords
    let expected_hash = hash_password(password);
    hash == expected_hash
}




impl User {
    async fn create(pool: Arc<PgPool>, username: &str, email: &str, password_hash: &str) -> Result<()> {
        sqlx::query!(
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)",
            username,
            email,
            password_hash
        )
        .execute(&*pool)
        .await?;
        Ok(())
    }

    async fn read(pool: Arc<PgPool>, user_id: i64) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, username, email, password_hash FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(&*pool)
        .await?;
        Ok(user)
    }

    async fn update(pool: Arc<PgPool>, user_id: i64, username: &str, email: &str, password_hash: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE users SET username = $1, email = $2, password_hash = $3 WHERE id = $4",
            username,
            email,
            password_hash,
            user_id
        )
        .execute(&*pool)
        .await?;
        Ok(())
    }

    async fn delete(pool: Arc<PgPool>, user_id: i64) -> Result<()> {
        sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&*pool)
            .await?;
        Ok(())
    }
}