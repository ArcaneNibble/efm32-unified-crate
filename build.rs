use std::io::Write;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let destination = std::path::Path::new(&out_dir).join("svd_bits.rs");
    let mut f = std::fs::File::create(&destination).unwrap();

    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let svd = std::path::Path::new(&root_dir).join("svd");
    let files = std::fs::read_dir(svd).unwrap();

    let mut frags = Vec::new();

    for file in files {
        let path = file.expect("failed to get path").path();
        let path_str = path.as_os_str().to_str().unwrap();
        if path_str.ends_with(".tmpl.svd") || path_str.ends_with(".frag") {
            let path = path.canonicalize().unwrap();

            let filename = path.file_name().unwrap().to_str().unwrap().to_owned();
            let id_string = filename.chars().map(|x| match x {
                'A'...'Z' | 'a'...'z' | '0'...'9' => x,
                _ => '_',
            }).collect::<String>().to_ascii_uppercase();

            write!(f, r#"
                const {}: &'static [u8] = include_bytes!("{}");
                "#, id_string, path.to_str().unwrap()).unwrap();

            if path_str.ends_with(".frag") {
                frags.push((id_string, filename));
            }
        }
    }

    write!(f, r#"const ALL_FRAGS: &'static [(&'static [u8], &'static str)] = &["#).unwrap();
    for (frag_id, frag_fn) in frags {
        write!(f, r#"(&{}, "{}"),"#, frag_id, frag_fn).unwrap();
    }
    write!(f, r#"];"#).unwrap();
}
