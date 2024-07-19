mod user_dto;
mod chat_info_dto;
mod message_dto;
mod chat_dto;

pub use user_dto::*;
pub use chat_info_dto::*;
pub use message_dto::*;
pub use chat_dto::*;

pub trait ObjectToDTO<Input>
{
    fn obj_to_dto(model_input: Input) -> Self;
}

pub fn vec_to_dto<Input, Output>(input_vec: Vec<Input>) -> Vec<Output>
where 
    Output: ObjectToDTO<Input>
{
    let mut dtos: Vec<Output> = Vec::new();

    for input in input_vec
    {
        dtos.push(Output::obj_to_dto(input))
    }
    
    dtos
}