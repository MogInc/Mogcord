use crate::model::user::UserFlag;


#[derive(Clone, Debug)]
pub struct Ctx
{
	user_id: String,
	user_flag: UserFlag,
}

impl Ctx 
{
	pub fn new(user_id: String, user_flag: UserFlag) -> Self 
    {
		Self 
        { 
            user_id: user_id,
			user_flag: user_flag,
        }
	}
}

impl Ctx 
{
	pub fn user_id(self) -> String 
    {
		self.user_id
	}

	pub fn user_id_ref(&self) -> &String 
    {
		&self.user_id
	}

	pub fn user_flag(self) -> UserFlag
    {
		self.user_flag
	}

	pub fn user_flag_ref(&self) -> &UserFlag 
    {
		&self.user_flag
	}
}