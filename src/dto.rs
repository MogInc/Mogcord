mod chat;
mod channel;
mod message;
mod user;
mod server;

pub use chat::*;
pub use channel::*;
pub use message::*;
pub use user::*;
pub use server::*;

pub trait ObjectToDTO<Input>
{
    fn obj_to_dto(model_input: Input) -> Self;
}

#[must_use]
pub fn vec_to_dto<Input, Output>(input_vec: Vec<Input>) -> Vec<Output>
where 
    Output: ObjectToDTO<Input>
{
    let mut dtos: Vec<Output> = Vec::new();

    for input in input_vec
    {
        dtos.push(Output::obj_to_dto(input));
    }
    
    dtos
}