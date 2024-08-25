mod flag;
mod repository;

pub use flag::*;
pub use repository::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User
{
    pub id: String,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub flag: Flag,
}

impl User
{
    #[must_use]
    pub fn convert(
        id: String,
        username: String,
        email: String,
        hashed_password: String,
        flag: Flag,
    ) -> Self
    {
        Self {
            id,
            username,
            email,
            hashed_password,
            flag,
        }
    }
    #[must_use]
    pub fn new(
        username: String,
        email: String,
        hashed_password: String,
    ) -> Self
    {
        Self {
            id: Uuid::now_v7().to_string(),
            username,
            email,
            hashed_password,
            flag: Flag::None,
        }
    }
}

impl std::hash::Hash for User
{
    fn hash<H: std::hash::Hasher>(
        &self,
        state: &mut H,
    )
    {
        self.id.hash(state);
    }
}

impl PartialEq for User
{
    fn eq(
        &self,
        other: &Self,
    ) -> bool
    {
        self.id == other.id
    }
}
impl Eq for User {}

#[cfg(test)]
mod tests
{
    use uuid::Uuid;

    use crate::model::user::{Flag, User};

    #[test]
    fn test_convert_user_is_valid()
    {
        let id = String::from("12345678");
        let username = String::from("Gwilom");
        let email = String::from("ElGoblino@example.com");
        let hashed_password = String::from("fake_hashed_password");
        let user_flag = Flag::None;

        let user: User = User::convert(
            id.clone(),
            username.clone(),
            email.clone(),
            hashed_password.clone(),
            user_flag.clone(),
        );

        assert_eq!(id, user.id);
        assert_eq!(username, user.username);
        assert_eq!(email, user.email);
        assert_eq!(
            hashed_password,
            user.hashed_password
        );
        assert_eq!(user_flag, user.flag);
    }

    #[test]
    fn test_new_user_is_valid()
    {
        let username: String = String::from("Gwilom");
        let email: String = String::from("ElGoblino@example.com");
        let hashed_password: String = String::from("fake_hashed_password");

        let user: User = User::new(
            username.clone(),
            email.clone(),
            hashed_password.clone(),
        );

        assert!(Uuid::parse_str(&user.id).is_ok());
        assert_eq!(username, user.username);
        assert_eq!(email, user.email);
        assert_eq!(
            hashed_password,
            user.hashed_password
        );
        assert_eq!(Flag::None, user.flag);
    }
}
