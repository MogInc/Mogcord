use crate::model::error;

impl error::Client
{
    #[must_use]
    pub fn translate_error<'b>(&self) -> &'b str
    {
        match self
        {
            error::Client::CHAT_ALREADY_EXISTS => "Chat already exists.",
            error::Client::CHAT_CANT_GAIN_USERS => "Chat cant gain any users.",
            error::Client::CHAT_ADD_NON_FRIEND => "Cant add strangers to a chat.",
            error::Client::CHAT_ADD_WITH_SELF => "You're already in this chat.",
            error::Client::INVALID_PARAMS => "Invalid parameters.",
            error::Client::MAIL_IN_USE => "email already in use.",
            error::Client::MESSAGE_NOT_PART_CHANNEL => "This message doesnt belong here",
            error::Client::NOT_ALLOWED_PLATFORM =>
                "You're not allowed on this platform anymore, contact support for more info.",
            error::Client::CHAT_EDIT_NOT_OWNER => "You dont have the permissions to edit this chat",
            error::Client::CHAT_PARENT_CTX_NOT_PART_OF_PARENT =>
                "You're not part of this channel parent.",
            error::Client::CHAT_CTX_NOT_PART_OF_CHAT => "You're not part of this chat.",
            error::Client::SERVER_CTX_NOT_PART_OF_SERVER => "You're not part of this server.",
            error::Client::PASSWORD_CONFIRM_NOT_MATCH => "Passwords do not match.",
            error::Client::PERMISSION_NO_ADMIN =>
                "You dont have permissions to acces this resource, please refrain from using this.",
            error::Client::PERMISSION_NO_AUTH => "Please re-authenticate.",
            error::Client::PRIVATE_CHAT_TRY_EDIT => "Private chats cant be edited.",
            error::Client::COOKIES_NOT_FOUND => "You're missing certain cookies.",
            error::Client::MESSAGE_CREATE_FAIL => "Failed to create message.",
            error::Client::MESSAGE_EDIT_FAIL => "Failed to edit message.",
            error::Client::SERVER_BLOCKED_YOU => "Server has you blocked.",
            error::Client::SERVER_NOT_FOUND => "Server you're trying to reach doesn't exist.",
            error::Client::SERVICE_ERROR => "Eh oh.",
            error::Client::RELATION_NO_INCOMING_FRIEND =>
                "There seems to be no incoming friend request from that user.",
            error::Client::RELATION_DUPLICATE_OUTGOING_FRIEND =>
                "You've already send a friend request.",
            error::Client::RELATION_SELF_TRY_BLOCK_SELF => "Can't block yourself.",
            error::Client::RELATION_SELF_TRY_FRIEND_SELF => "Can't add yourself as a friend.",
            error::Client::RELATION_SELF_TRY_UNBLOCK_SELF => "Can't unblock yourself.",
            error::Client::RELATION_SELF_TRY_UNFRIEND_SELF => "Can't unfriend yourself.",
            error::Client::USER_ALREADY_LOGGED_IN => "User already logged in.",
            error::Client::USERNAME_IN_USE => "Username is already in use.",
            error::Client::RELATION_USER_ALREADY_BLOCKED => "This user is already blocked.",
            error::Client::RELATION_USER_ALREADY_FRIEND => "This user is already your friend.",
            error::Client::RELATION_USER_BLOCKED => "This user is blocked.",
            error::Client::RELATION_USER_BLOCKED_YOU => "This user has you blocked.",
        }
    }
}

#[derive(PartialEq)]
pub enum PotentialErrorDisplay
{
    None,
    Alert,
}

pub struct HtmxError
{
    pub client: error::Client,
    pub display: PotentialErrorDisplay,
}

impl HtmxError
{
    #[must_use]
    pub fn new(client: error::Client) -> Self
    {
        Self {
            client,
            display: PotentialErrorDisplay::None,
        }
    }
    #[must_use]
    pub fn new_form_error(client: error::Client) -> Self
    {
        Self {
            client,
            display: PotentialErrorDisplay::Alert,
        }
    }
}