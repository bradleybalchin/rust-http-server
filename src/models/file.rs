use serde::Serialize;

// file data model

#[derive(Serialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
}

#[derive(Serialize)]
pub struct  FileList {
    pub files: Vec<FileInfo>
}