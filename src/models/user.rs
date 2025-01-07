use crate::{AppError, AppState};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigninUser {
    pub email: String,
    pub password: String,
}

impl AppState {
    pub async fn create_user(&self, input: CreateUser) -> Result<User, AppError> {
        if self.find_user_by_email(&input.email).await?.is_some() {
            return Err(AppError::ServerError(format!(
                "user by {} is already exists",
                input.email
            )));
        }

        let password_hash = hash_password(&input.password)?;

        let user = sqlx::query_as(
            "
            insert into users (username, email, password_hash) values ($1, $2, $3) RETURNING *
            ",
        )
        .bind(input.username)
        .bind(input.email)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as(
            "
            select * from users where email = $1
            ",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn signin(&self, input: SigninUser) -> Result<User, AppError> {
        match self.find_user_by_email(&input.email).await? {
            Some(user) => {
                if verify_password(&input.password, &user.password_hash)? {
                    Ok(user)
                } else {
                    Err(AppError::ServerError(format!(
                        "user by {} password error",
                        user.email
                    )))
                }
            }
            None => Err(AppError::ServerError(format!(
                "user by {} not exists",
                input.email
            ))),
        }
    }
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();

    let password_hash = PasswordHash::new(password_hash)?;

    let is_valid = argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok();

    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    impl SigninUser {
        fn new(email: impl Into<String>, password: impl Into<String>) -> Self {
            Self {
                email: email.into(),
                password: password.into(),
            }
        }
    }

    impl CreateUser {
        fn new(
            username: impl Into<String>,
            email: impl Into<String>,
            password: impl Into<String>,
        ) -> Self {
            Self {
                username: username.into(),
                email: email.into(),
                password: password.into(),
            }
        }
    }

    #[test]
    fn password_hash_and_verify_password_should_work() -> Result<()> {
        let password = "hunter21";
        let password_hash = hash_password(password)?;
        let ret = verify_password(password, &password_hash)?;
        assert!(ret);

        let password_hash = hash_password("123456")?;
        let ret = verify_password(password, &password_hash)?;
        assert!(!ret);

        Ok(())
    }

    #[tokio::test]
    async fn create_user_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let username = "TeamMeng";
        let email = "Meng@123.com";
        let password = "hunter21";

        let input = CreateUser::new(username, email, password);

        let user = state.create_user(input.clone()).await?;

        assert_eq!(username, user.username);
        assert_eq!(email, user.email);

        let ret = verify_password(password, &user.password_hash)?;
        assert!(ret);

        let ret = state.create_user(input).await;

        assert!(ret.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn find_user_by_email_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let username = "TeamMeng";
        let email = "Meng@123.com";
        let password = "hunter21";

        let input = CreateUser::new(username, email, password);

        state.create_user(input).await?;

        let user = state
            .find_user_by_email(email)
            .await?
            .expect("user should exists");

        assert_eq!(username, user.username);
        assert_eq!(email, user.email);

        let ret = verify_password(password, &user.password_hash)?;
        assert!(ret);

        let ret = state.find_user_by_email("Alice@123.com").await?;

        assert!(ret.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn signin_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let username = "TeamMeng";
        let email = "Meng@123.com";
        let password = "hunter21";

        let input = CreateUser::new(username, email, password);

        state.create_user(input).await?;

        let input = SigninUser::new(email, password);

        let user = state.signin(input).await?;

        assert_eq!(username, user.username);
        assert_eq!(email, user.email);

        let ret = verify_password(password, &user.password_hash)?;
        assert!(ret);

        Ok(())
    }
}
