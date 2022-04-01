
use serde::{Serialize, Deserialize};
// make all the required stuff for a filesystem, but with lifetimes
#[derive(Serialize, Deserialize)]
pub struct FileSystem {
    pub root: String,
    pub files: Vec<String>,
    pub folders: Vec<String>
}
impl FileSystem {
    pub fn new(root: String) -> FileSystem {
        FileSystem {
            root: root,
            files: Vec::new(),
            folders: Vec::new()
        }
    }
    pub fn add_file(&mut self, file: String) {
        self.files.push(file);
    }
    pub fn add_folder(&mut self, folder: String) {
        self.folders.push(folder);
    }
}

struct Folder<'a> {
    name: &'a str,
    files: Vec<&'a str>,
    folders: Vec<&'a str>
} 

impl <'a>Folder<'a> {
    fn new(name: &'a str)->Folder<'a> {
        Folder {
            name: name,
            files: Vec::new(),
            folders: Vec::new()
        }
    }
    fn add_file(&mut self, file: &'a str){
        self.files.push(file);
    }
    fn add_folder(&mut self, folder: &'a str) {
        self.folders.push(folder);
    }
}

struct File<'a> {
    name: &'a str,
    content: &'a str
}

impl<'a> File<'_> {
    fn new(name: &'a str, content: &'a str) -> File<'a> {
        File {
            name: name,
            content: content
        }
    }
}