use crate::model::user::UserFlag;


#[derive(Clone, Debug)]
pub struct Ctx
{
	user_id: String,
	user_flag: UserFlag,
}

impl Ctx 
{
	#[must_use]
	pub fn new(user_id: String, user_flag: UserFlag) -> Self 
    {
		Self 
        { 
            user_id,
			user_flag,
        }
	}
}

impl Ctx 
{
	#[must_use]
	pub fn user_id(self) -> String 
    {
		self.user_id
	}

	#[must_use]
	pub fn user_flag(self) -> UserFlag
    {
		self.user_flag
	}
}