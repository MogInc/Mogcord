#[derive(Clone, Debug)]
pub struct Ctx
{
    user_id: String,
    is_admin: bool,
}

impl Ctx
{
    #[must_use]
    pub fn new(
        user_id: String,
        is_admin: bool,
    ) -> Self
    {
        Self {
            user_id,
            is_admin,
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
    pub fn user_id_ref(&self) -> &str
    {
        &self.user_id
    }

    #[must_use]
    pub fn is_admin(&self) -> bool
    {
        self.is_admin
    }
}
