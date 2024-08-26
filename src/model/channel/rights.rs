use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum_macros::EnumCount;

#[derive(Clone, Debug, Serialize, Deserialize, EnumIter, EnumCount)]
//channel rights
pub enum Rights
{
    Read(Option<bool>),
    Write(Option<bool>),
}

impl Rights
{
    fn name(&self) -> &str
    {
        match self
        {
            Rights::Read(_) => "read",
            Rights::Write(_) => "write",
        }
    }
}

impl PartialEq for Rights
{
    fn eq(&self, other: &Self) -> bool
    {
        self.name() == other.name()
    }
}
impl Eq for Rights {}
