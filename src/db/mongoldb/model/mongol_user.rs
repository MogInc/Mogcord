use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use crate::{db::mongoldb::MongolDB, model::user::User};

#[derive(Debug, Serialize, Deserialize)]
pub struct MongolUser
{
    pub _id : ObjectId,
    pub uuid: String,
    pub name: String,
    pub mail: String,
}

impl MongolUser
{
    pub async fn convert_to_db(
        user : &User, 
        mongol_db_option : Option<&MongolDB>
    ) -> MongolUser
    {

        if let Some(mongol_db) = mongol_db_option 
        {
            match mongol_db.get_user_db_object_by_id(&user.uuid).await
            {
                Ok(user_from_db) => 
                {
                    return MongolUser
                    {
                        _id: user_from_db._id.clone(),
                        uuid: user_from_db.uuid.clone(),
                        name: user_from_db.name.clone(),
                        mail: user_from_db.mail.clone()
                    }
                },
                Err(_) => return Self::new_mongol_user(&user),
            }
        }
        return Self::new_mongol_user(&user)
    }

    pub fn convert_to_domain(self) -> User
    {
        User::convert(self.uuid, self.name, self.mail)
    }

    fn new_mongol_user(user : &User) ->  MongolUser
    {
        MongolUser
        {
            _id: ObjectId::new(),
            uuid: user.uuid.clone(),
            name: user.name.clone(),
            mail: user.mail.clone()
        }
    }
}