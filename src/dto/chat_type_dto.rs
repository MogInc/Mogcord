use serde::Serialize;

use crate::model::chat::ChatType;

use super::ObjectToDTO;

#[derive(Serialize)]
pub struct ChatTypeDTO
{
    name: Option<String>,
    owners: Vec<String>,
    users: Vec<String>,
}

impl ObjectToDTO<ChatType> for ChatTypeDTO
{
    fn obj_to_dto(model_input: ChatType) -> Self 
    {
        match model_input
        {
            ChatType::Private { owners } => 
            {
                Self
                {
                    name: None,
                    owners: owners.into_iter().map(|user| user.id).collect(),
                    users: Vec::new(),
                }
            },
            ChatType::Group { name, owner, users } => 
            {
                Self
                {
                    name: Some(name),
                    owners: vec![owner.id],
                    users: users.into_iter().map(|user| user.id).collect(),
                }
            },
            ChatType::Server { name, owner, users } => 
            {
                Self
                {
                    name: Some(name),
                    owners: vec![owner.id],
                    users: users.into_iter().map(|user| user.id).collect(),
                }
            },
        }
    }
}