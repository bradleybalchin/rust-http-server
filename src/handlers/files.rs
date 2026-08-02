
use std::{fs::{self, DirEntry}, path::Path};
use crate::models::{file::FileInfo, file::FileList, file::Directory};
use axum::{
    http::StatusCode,
    Json,
};


// recursively read directories in a path
fn read_dir(path: &Path) -> Result<Directory, StatusCode> {
    let mut directory = Directory{
        name:path.file_name()
        .map(|n| n.to_string_lossy().into_owned() )
        .unwrap_or_else(|| path.to_string_lossy().into_owned()), //fallback to path if Option is none

        path: path.to_string_lossy().into_owned(),
        directories: Vec::new(),
        files: Vec::new()

    };

    // iterate through directry entries
    for entry in fs::read_dir(path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
    {
        let entry:DirEntry = entry.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let entry_path = entry.path();

        // TODO: crawl through subdirectories (walkdir and use tree structure)

        if entry_path.is_dir() { // recursively define directories
            directory.directories.push(read_dir(&entry_path)?);
        }
        else if entry_path.is_file() { // get file data and ad to directories files
            let metadata = entry.metadata().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            directory.files.push(FileInfo { 
                name: entry.file_name().to_string_lossy().into_owned(), 
                path: entry_path.to_string_lossy().into_owned(), 
                size: metadata.len() });

            
        }
    }

    Ok(directory)


}

// list files in docs
pub async fn list() -> Result<Json<Directory>, StatusCode> {

    println!("List files");
    let directory = read_dir(Path::new("storage/docs"))?;
    

    Ok(Json(directory))

}