use std::io;
use std::path::Path;
use std::fs::File;

use rocket::Data;
use paste_id::PasteID;


#[get("/<id>")]
pub fn retrieve(id: PasteID) -> Option<File> {
    File::open(&format!("upload/{}", id)).ok()
}


#[post("/", data = "<paste>")]
pub fn upload(paste: Data) -> io::Result<String> {
    let id = PasteID::new();
    let filename = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);

    println!("file: {}, uri: {}", filename, url);

    // Write the paste out to the file and return the URL.
    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
}
