use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::UserFlag;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct User
{
    pub id: String,
    pub username: String,
    pub mail: String,
    pub hashed_password: String,
    pub flag: UserFlag,
}

impl User
{
    pub fn convert(id: String, username: String, mail: String, hashed_password: String, user_flag: UserFlag) -> Self
    {
        Self
        {
            id,
            username,
            mail,
            hashed_password,
            flag: user_flag,
        }
    }

    pub fn new(username: String, mail: String, hashed_password: String) -> Self
    {
        Self
        {
            id: Uuid::now_v7().to_string(),
            username,
            mail,
            hashed_password,
            flag: UserFlag::None,
        }
    }
}

#[cfg(test)]
mod tests 
{
    use uuid::Uuid;

    use crate::model::user::{User, UserFlag};
    
    #[test]
    fn test_convert_user_is_valid() 
    {
        let id = String::from("12345678");
        let username = String::from("Gwilom");
        let mail = String::from("ElGoblino@example.com");
        let hashed_password = String::from("fake_hashed_password");
        let user_flag = UserFlag::None;

        let user: User = User::convert(id.clone(), username.clone(), mail.clone(), hashed_password.clone(), user_flag.clone());

        assert_eq!(id, user.id);
        assert_eq!(username, user.username);
        assert_eq!(mail, user.mail);
        assert_eq!(hashed_password, user.hashed_password);
        assert_eq!(user_flag, user.flag);
    }

    #[test]
    fn test_new_user_is_valid() 
    {
        let username: String = String::from("Gwilom");
        let mail: String = String::from("ElGoblino@example.com");
        let hashed_password: String = String::from("fake_hashed_password");

        let user: User = User::new(username.clone(), mail.clone(), hashed_password.clone());

        assert!(Uuid::parse_str(&user.id).is_ok());
        assert_eq!(username, user.username);
        assert_eq!(mail, user.mail);
        assert_eq!(hashed_password, user.hashed_password);
        assert_eq!(UserFlag::None, user.flag);
    }
}