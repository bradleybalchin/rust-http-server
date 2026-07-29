
use std::{fs::{self, DirEntry}};
use crate::models::{file::FileInfo, file::FileList};
use axum::{
    http::StatusCode,
    Json,
};


// list files in docs
pub async fn list() -> Result<Json<FileList>, StatusCode> {

    println!("List files");
    let mut filelist = FileList {
        files: Vec::new()
    };

    for entry in fs::read_dir("public/docs")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? 
    {
        let entry:DirEntry = entry.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let path = entry.path();

        // TODO: crawl through subdirectories

        if path.is_file() {
            let metadata = entry.metadata().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            filelist.files.push(FileInfo { 
                name: entry.file_name().to_string_lossy().into_owned(), 
                path: path.to_string_lossy().into_owned(), 
                size: metadata.len() });
        }

    }
    

    Ok(Json(filelist))

}