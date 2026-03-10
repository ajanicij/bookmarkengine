// use netscape_bookmark_parser;
use netscape_bookmark_parser::generate_json_from_html::run;

fn main() -> std::io::Result<()> {
//    netscape_bookmark_parser::run("input.html", "output_directory")?;
    run("input.html", "output_directory")?;
    Ok(())
}
