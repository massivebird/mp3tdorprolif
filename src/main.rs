use clap::Arg;
use id3::{Tag, TagLike};

// MP3 ID3 tags: https://exiftool.org/TagNames/ID3.html

fn main() {
    let matches = clap::command!()
        .arg(
            Arg::new("path")
                .required(true)
                .value_name("PATH")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .help("Path to music library root"),
        )
        .get_matches();

    let root = {
        let path = matches.get_one::<String>("path").unwrap().to_string();
        std::fs::read_dir(path).expect("Failed to open specified directory")
    };

    macro_rules! only_ok_directories_in {
        ( $path: expr) => {
            $path
                .filter_map(Result::ok)
                .filter(|d| d.file_type().unwrap().is_dir())
                .map(|d| (d.file_name(), std::fs::read_dir(d.path())))
        };
    }

    let mut num_fixed_files = 0;

    for (_, artist_path) in only_ok_directories_in!(root) {
        for (_, album_path) in only_ok_directories_in!(artist_path.unwrap()) {
            for file in album_path.unwrap().filter_map(Result::ok) {
                // skip non-mp3s
                if !file.path().extension().is_some_and(|e| e == "mp3") {
                    continue;
                }

                if fix_mp3(&file) {
                    println!("Fixed file {file:?}");
                    num_fixed_files += 1;
                };
            }
        }
    }

    println!("Done! Fixed {num_fixed_files} files.");
}

fn fix_mp3(file: &std::fs::DirEntry) -> bool {
    let Ok(mut tag) = Tag::read_from_path(file.path()) else {
        println!("WARNING: failed to read tags from {file:?}");
        return false;
    };

    // skip fixed files
    if tag.frames().any(|f| f.id() == "TDOR")
        && tag.frames().any(|f| f.id() == "TDRC")
    {
        return false;
    };

    let Some(release_year) = tag
        .frames()
        .find(|f| matches!(f.id(), "TDOR" | "TDRC" | "TDRL"))
        .map(|f| f.content().text().unwrap().to_owned())
    else {
        // failed to find a release date
        return false;
    };

    tag.add_frame(id3::Frame::text("TDOR", &release_year));
    // TDRC -> "Recording Time"
    tag.add_frame(id3::Frame::text("TDRC", &release_year));
    // TDRL -> "Release Time"
    tag.add_frame(id3::Frame::text("TDRL", &release_year));

    tag.write_to_path(file.path(), id3::Version::Id3v24)
        .unwrap();

    true
}
