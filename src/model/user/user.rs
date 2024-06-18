use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub enum UserFlag
{
    None,
    Disabled,
    Deleted { date: DateTime<Utc> },
    Banned { date: DateTime<Utc> },
    Admin,
    Owner,
}

#[derive(Serialize, Deserialize)]
pub struct User
{
    pub uuid: String,
    pub name: String,
    pub mail: String,
}

impl User
{
    pub fn convert(uuid: String, name: String, mail: String) -> Self
    {
        User
        {
            uuid,
            name,
            mail
        }
    }

    pub fn new(name: String, mail: String) -> Self
    {
        User
        {
            uuid: Uuid::new_v4().to_string(),
            name,
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
    fn test_convert_user_is_valid() {
        let uuid = String::from("12345678");
        let name = String::from("Gwilom");
        let mail = String::from("ElGoblino@example.com");

        let user = User::convert(uuid.clone(), name.clone(), mail.clone());

        assert_eq!(user.uuid, uuid);
        assert_eq!(user.name, name);
        assert_eq!(user.mail, mail);
    }

    #[test]
    fn test_new_user_is_valid() {
        let name = String::from("Gwilom");
        let mail = String::from("ElGoblino@example.com");

        let user = User::new(name.clone(), mail.clone());

        assert!(Uuid::parse_str(&user.uuid).is_ok());
        assert_eq!(user.name, name);
        assert_eq!(user.mail, mail);
    }
}