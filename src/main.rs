use id3::{Tag, TagLike};

fn main() {
    let path = std::path::Path::new("/home/penguino/Music/24thankyou/Everything I Was, Burning Slow/01 - 24thankyou - Everything I Was, Burning Slow.mp3");

    let mut tag = Tag::read_from_path(path).unwrap();

    // skip fixed files
    if tag.frames().any(|f| f.id() == "TDRC") {
        return;
    };

    let Some(release_year) = tag
        .frames()
        .find(|f| f.id() == "TDOR")
        .map(|f| f.content().text().unwrap().to_owned())
    else {
        return;
    };

    // TDRC -> "Recording Time"
    tag.add_frame(id3::Frame::text("TDRC", &release_year));
    // TDRL -> "Release Time"
    tag.add_frame(id3::Frame::text("TDRL", &release_year));

    tag.write_to_path(path, id3::Version::Id3v22).unwrap();
}
