use std::fs::{
    read_dir,
};
use std::path::Path;
use std::io::Result;

pub struct DirectoryContent {
    pub path: String,
    pub files: Vec<String>,
    pub directories: Vec<String>
}

pub fn get_directory_content(path:&str) -> Result<DirectoryContent>{
        let directory_content = read_dic_content(String::from(path));
        let directory_content = directory_content?;
        let mut files:Vec<String> = Vec::new();
        let mut other:Vec<String> = Vec::new();
        for content in directory_content{
            if content.1 == 0{
                files.push(content.0);
            } else {
                other.push(content.0);
            }
        }
        return Ok(DirectoryContent {
            path: String::from(path),
            files: files,
            directories: other
        });
}

pub fn check_directory(path: &str) -> bool{
    let check_path = Path::new(path);
    return check_path.is_dir();
}

/*
    Return Vector with filenames and filtetype
       filetye: 0 -> file, 1 -> Directory or Symlink
 */
fn read_dic_content(path:String) -> Result<Vec<(String,i8)>> {
    let mut entry_names: Vec<(String,i8)> = Vec::new();
    let entries = read_dir(path)?;
    for entry in entries{
        let entry = entry?;
        let name = entry.file_name().into_string().unwrap_or_default();
        let file_type = entry.file_type()?;
        if file_type.is_file(){
            entry_names.push((name,0));
        } else{
            entry_names.push((name,1));
        }
    }
    Ok(entry_names)
 }