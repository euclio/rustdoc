extern crate rustdoc;
extern crate jsonapi;

mod validation_tests {
    use jsonapi::api::JsonApiDocument;
    use std::fs::File;
    use std::io::Read;
    use std::path::{Path, PathBuf};
    use rustdoc::{build, Config};

    #[test]
    fn json_fmt_test() {
        let config = Config::new(PathBuf::from("example"), vec![]).unwrap_or_else(|err| {
            panic!("Couldn't create the config: {}", err);
        });

        build(&config, &["json"]).unwrap_or_else(|err| {
            panic!("Error: {}", err);
        });

        let path = Path::new("example/target/doc/data.json");

        let mut file = File::open(&path).unwrap_or_else(|err| {
            panic!("couldn't open data.json: {}", err);
        });

        let mut s = String::new();
        file.read_to_string(&mut s).unwrap_or_else(|err| {
            panic!("Error: {}", err);
        });

        let doc = JsonApiDocument::from_str(&s).unwrap_or_else(|err| {
            panic!("Error: {}", err);
        });

        match doc.validate() {
            Some(errors) => panic!("Error: {:?}", errors),
            None => (),
        }
    }
}