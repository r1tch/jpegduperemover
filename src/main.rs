/*
fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}
*/


use rexif;
use rexif::ExifTag;
use std::collections::hash_set::HashSet;
use std::env;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

static USAGE: &str = "Usage: jpegduperemover [-v] <backupdir> <dupedir>";

fn parse_args() -> (bool, String, String) {
    let mut args = env::args();
    args.next();

    let mut verbose = false;
    let mut backupdir = args.next().expect(USAGE);
    if backupdir == "-v" {
        verbose = true;
        backupdir = args.next().expect(USAGE);
    }
    let dupedir = args.next().expect(USAGE);
    (verbose, backupdir, dupedir)
}

#[derive(Debug)]
struct JpegImage {
    fullpath: PathBuf,
    size: u64,
    exifdate: String,
}

impl JpegImage {
    /*fn name(&self) -> String {
        String::from(self.fullpath.file_name().unwrap().to_str().unwrap())
    }*/

    fn get_exif_date_for(path: &Path) -> String {
        if let Ok(exif_data) = rexif::parse_file(path) {
            // println!("ExifData for {:?}:", path);

            for e in exif_data.entries {
                if e.tag == ExifTag::DateTime
                    || e.tag == ExifTag::DateTimeOriginal
                    || e.tag == ExifTag::DateTimeDigitized
                {
                    if let rexif::TagValue::Ascii(string) = e.value {
                        return string;
                    }
                }
            }
            return "MISSING EXIF DATE".to_string();
        }
        "NO EXIF DATA".to_string()
    }

    fn from_dir_entry(dir_entry: &walkdir::DirEntry) -> JpegImage {
        JpegImage {
            fullpath: dir_entry.path().to_path_buf(),
            size: dir_entry.metadata().expect("cannot get metadata").len(),
            exifdate: Self::get_exif_date_for(dir_entry.path()),
        }
    }
}

impl PartialEq for JpegImage {
    fn eq(&self, other: &JpegImage) -> bool {
        self.size == other.size && self.exifdate == other.exifdate
    }
}

impl Eq for JpegImage {}

impl Hash for JpegImage {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.size.hash(state);
        self.exifdate.hash(state);
    }
}

fn is_jpeg(dir_entry: &walkdir::DirEntry) -> bool {
    dir_entry
        .file_name()
        .to_str()
        .map(|s| s.to_lowercase().ends_with(".jpg") || s.to_lowercase().ends_with(".jpeg"))
        .unwrap_or(false)
}

fn collect_images_recursively(dirname: &str) -> HashSet<JpegImage> {
    WalkDir::new(dirname)
        .into_iter()
        .filter_entry(|dir_entry| dir_entry.file_type().is_dir() || is_jpeg(&dir_entry))
        .filter_map(|e| e.ok())
        .filter(|dir_entry| is_jpeg(&dir_entry))
        .map(|dir_entry| JpegImage::from_dir_entry(&dir_entry))
        .collect()
}

fn main() {
    let (verbose, backupdir, dupedir) = parse_args();

    // println!("Collecting backed-up images...");
    let backup_images = collect_images_recursively(&backupdir);
    // println!("Collecting duplicate images...");
    let dupe_images = collect_images_recursively(&dupedir);

    for dupe_image in dupe_images {
        if let Some(backup_image) = backup_images.get(&dupe_image) {
            if let Some(dupe_str_path) = dupe_image.fullpath.to_str() {
                if verbose {
                    if let Some(backup_str_path) = backup_image.fullpath.to_str() {
                        println!("# \"{}\" \"{}\" ", dupe_str_path, backup_str_path);
                    }
                }
                println!("rm \"{}\"", dupe_str_path);
            }
        }
    }

    /*
    //let intersection = dupe_images.intersection(&backup_images);
    let intersection = backup_images.intersection(&dupe_images);
    println!("Type: {}", type_of(&intersection));
    // here we print "rm <filename>" for all files that also exist in dupeImages.
    for jpeg_image in intersection {
        if let Some(str_path) = jpeg_image.fullpath.to_str() {
            println!("rm {}", str_path);
        }
    }
    */

}
