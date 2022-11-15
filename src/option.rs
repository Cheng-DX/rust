pub mod file {
    use std::fs;

    pub fn test() {
        let content = match fs::read_to_string("src/option.rs") {
            Ok(content) => content,
            Err(error) => {
                println!("Problem opening the file: {:?}", error);
                String::from("")
            }
        };
        println!("content is:\n {content}");
    }
}
