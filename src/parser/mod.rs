pub mod content;

// let file = fs::read_to_string("tmp/in.md");
// let markdown_input = file.expect("oops!");
//
// let options = Options::all();
// let parser = Parser::new_ext(&markdown_input, options);
//
// let mut html_output = fs::OpenOptions::new()
// .write(true)
// .create(true)
// .open("tmp/out.html")
// .unwrap();
// html::write_html(&mut html_output, parser).unwrap();