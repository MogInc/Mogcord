pub mod log;

pub struct FileWriter<'a>
{
    folder_path: &'a str,
}

impl<'a> FileWriter<'a>
{
    #[must_use]
    pub fn new(folder_path: &'a str) -> Self
    {
        Self
        {
            folder_path
        }
    }
}