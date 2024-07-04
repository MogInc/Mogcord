use serde::{Serialize, Deserialize};
use uuid::Uuid;



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User
{
    pub id: String,
    pub username: String,
    pub mail: String,
}

impl User
{
    pub fn convert(id: String, username: String, mail: String) -> Self
    {
        User
        {
            id,
            username,
            mail
        }
    }

    pub fn new(username: String, mail: String) -> Self
    {
        User
        {
            id: Uuid::now_v7().to_string(),
            username,
            mail
        }
    }
}

#[cfg(test)]
mod tests 
{
    use uuid::Uuid;

    use crate::model::user::User;
    
    #[test]
    fn test_convert_user_is_valid() 
    {
        let id: String = String::from("12345678");
        let username: String = String::from("Gwilom");
        let mail: String = String::from("ElGoblino@example.com");

        let user: User = User::convert(id.clone(), username.clone(), mail.clone());

        assert_eq!(id, user.id);
        assert_eq!(username, user.username);
        assert_eq!(mail, user.mail);
    }

    #[test]
    fn test_new_user_is_valid() 
    {
        let username: String = String::from("Gwilom");
        let mail: String = String::from("ElGoblino@example.com");

        let user: User = User::new(username.clone(), mail.clone());

        assert!(Uuid::parse_str(&user.id).is_ok());
        assert_eq!(username, user.username);
        assert_eq!(mail, user.mail);
    }
}