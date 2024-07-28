pub mod log;

pub struct FileWriter<'a>
{
    folder_path: &'a str,
}

impl<'a> FileWriter<'a>
{
    #[must_use]
    /// Creates a new instance of `FileWriter` and ensures the specified folder path exists.
    /// 
    /// # Parameters
    /// 
    /// - `folder_path`: A string slice that holds the path to the folder.
    ///  
    /// NOTE: This path is ensured to exist by creating the directory and any necessary parent directories if they do not already exist.
    /// # Returns
    /// 
    /// - A new instance of `FileWriter`.
    /// 
    /// # Panics
    /// 
    /// This function will panic if it fails to create the directory or any of its parent directories.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use mogcord::file_writer::FileWriter;
    /// 
    /// let path = "/path/to/folder";
    /// let fw = FileWriter::new(path);
    /// 
    /// assert_eq!(path, fw.get_path());
    /// ```
    pub fn new(folder_path: &'a str) -> Self
    {
        std::fs::create_dir_all(folder_path)
            .expect("failed to create");
        Self
        {
            folder_path
        }
    }
}

impl<'a> FileWriter<'a>
{
    #[must_use]
    pub fn get_path(&self) -> &str
    {
        self.folder_path
    }
}