mod channel;
mod chat;
mod message;
mod server;
mod user;

pub use channel::*;
pub use chat::*;
pub use message::*;
pub use server::*;
pub use user::*;

pub trait ObjectToDTO<Input>
{
    fn obj_to_dto(model_input: Input) -> Self;
}

#[must_use]
pub fn vec_to_dto<Input, Output>(input_vec: Vec<Input>) -> Vec<Output>
where
    Output: ObjectToDTO<Input>,
{
    let mut dtos: Vec<Output> = Vec::new();

    for input in input_vec
    {
        dtos.push(Output::obj_to_dto(input));
    }

    dtos
}
