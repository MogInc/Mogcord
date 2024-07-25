use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
//server rights
pub enum Rights
{
    ReadChannels(Option<bool>),
}

impl Rights
{
    #[must_use]
    fn name(&self) -> &str 
    {
        match self 
        {
            Rights::ReadChannels(_) => "read_channels",
        }
    }
}

impl std::hash::Hash for Rights
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) 
    {
        self.name().hash(state);
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