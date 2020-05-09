use nfd2::Response;

fn main() -> anyhow::Result<()> {
    let path = match nfd2::open_file_dialog(Some("tmx"), None)? {
        Response::Okay(file_path) => println!("File path = {:?}", file_path),
        Response::OkayMultiple(files) => println!("Files {:?}", files),
        Response::Cancel => println!("User canceled"),
    };

    Ok(())
}
