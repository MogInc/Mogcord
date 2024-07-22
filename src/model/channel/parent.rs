pub trait Parent
{
    fn can_read(&self, user: &str) -> bool;
    fn can_write(&self, user: &str) -> bool;
}