use std::env;
use std::fs::read_dir;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    println!("Writing tests to: {:?}", out_dir);
    write_tag_tests(&out_dir);
    write_database_tests(&out_dir);
}

fn write_tag_tests(path: &Path) {
    let destination = path.join("tests.rs");
    let mut test_file = File::create(&destination).unwrap();

    // write test file header, put `use`, `const` etc there
    write!(
        test_file,
        r#"// THIS FILE IS AUTOGENERATED - DO NOT EDIT!
use seratodj::tag::format::id3::ID3Tag;
use seratodj::tag::format::flac::FLACTag;
use seratodj::tag::format::mp4::MP4Tag;
use seratodj::tag::format::ogg::OggTag;
"#
    )
    .unwrap();

    let test_data_directories = read_dir("./tests/data/tags").unwrap();
    for entry in test_data_directories {
        let directory = entry.unwrap().path().canonicalize().unwrap();
        if !directory.is_dir() {
            continue;
        }

        let tag_name = directory.file_name().unwrap();
        let tag_name = tag_name.to_str().expect("Failed to get tag name");

        for entry in directory.read_dir().unwrap() {
            let filepath = entry.unwrap().path();
            if let Some(ext) = filepath.extension() {
                if ext != "bin" {
                    return;
                }
            }

            write_tag_test(&mut test_file, filepath.as_path(), tag_name);
        }
    }
}

fn write_tag_test(test_file: &mut File, filepath: &Path, tag_name: &str) {
    let stem = filepath.file_stem().unwrap().to_str().unwrap();
    let stem_split: Vec<&str> = stem.rsplitn(2, '.').collect();
    let tag_type: &str = stem_split[0];
    let filename: &str = stem_split[1];

    let test_name = format!("serato_{}_{}_{}", tag_name, filename, tag_type);

    let parser = match tag_name {
        "analysis" => "Analysis",
        "autotags" => "Autotags",
        "beatgrid" => "Beatgrid",
        "markers" => "Markers",
        "markers2" => "Markers2",
        "overview" => "Overview",
        "vidassoc" => "VidAssoc",
        "relvolad" => "RelVolAd",
        _ => {
            panic!("Unknown tag name!")
        }
    };

    write!(
        test_file,
        include_str!("./tests/test_parse.rs.in"),
        name = test_name,
        filepath = filepath.to_str().unwrap(),
        parser = parser,
        tag_type = tag_type,
    )
    .unwrap();
}

fn write_database_tests(path: &Path) {
    let destination = path.join("database_tests.rs");
    let mut test_file = File::create(&destination).unwrap();

    // write test file header, put `use`, `const` etc there
    write!(
        test_file,
        r#"// THIS FILE IS AUTOGENERATED - DO NOT EDIT!
use serato_tags::library::database;
"#
    )
    .unwrap();

    let test_data_directories = read_dir("./tests/data/library").unwrap();
    for entry in test_data_directories {
        let directory = entry.unwrap().path().canonicalize().unwrap();
        if !directory.is_dir() {
            continue;
        }

        let db_name = directory.file_name().unwrap();
        let db_name = db_name.to_str().expect("Failed to get DB name");

        write_database_test(
            &mut test_file,
            directory.join("database V2").as_path(),
            db_name,
        );
    }
}

fn write_database_test(test_file: &mut File, filepath: &Path, db_name: &str) {
    let test_name = format!("serato_database_{}", db_name);
    write!(
        test_file,
        include_str!("./tests/database_parse.rs.in"),
        name = test_name,
        filepath = filepath.to_str().unwrap(),
    )
    .unwrap();
}
