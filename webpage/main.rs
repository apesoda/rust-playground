use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("index.html")?;

    // set vars to be expanded
    let domain = "example.com";
    let srv = "jonesy";
    let oob = format!("{}-out", srv);
    let oob_url = format!("https://{}.{}", oob, domain);

    // CREATE THE FILE
    writeln!(file, "<!DOCTYPE html>")?;
    writeln!(file, "<html>")?;
    writeln!(file, "<head><title>Rust Playground Webpage</title></head>")?;
    writeln!(file, "<body>")?;
    writeln!(file, "    <h1>The content below is made with variables :D</h1>")?;
    writeln!(file, "    <table>")?;
    writeln!(file, "    <tr><th>URL</th><td><a href='{}'>{}</a></td></tr>", oob_url, oob)?;
    writeln!(file, "    </table>")?;
    writeln!(file, "</body>")?;
    writeln!(file, "</html>")?;

    println!("HTML file created successfully!");
    Ok(())
}
