pub const CLI_COMMAND_NAME: &str = "sortfi";

pub const PICTURE_PATH: &str = "Pictures";
pub const PICTURE_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "webp", "svg", "ico", "bmp", "tiff", "tif", "avif", "heic",
    "heif", "jxl", "raw", "cr2", "cr3", "nef", "arw", "dng", "orf", "rw2", "pef", "raf", "psd",
    "psb", "ai", "eps", "xcf", "fig", "sketch", "cdr", "indd", "tga", "dds", "exr", "hdr",
];

pub const MUSIC_PATH: &str = "Music";
pub const MUSIC_EXTENSIONS: &[&str] = &[
    "mp3", "aac", "ogg", "m4a", "opus", "wma", "mp2", "amr", "ac3", "dts", "flac", "wav", "alac",
    "aiff", "aif", "ape", "wv", "tta", "mid", "midi", "mod", "s3m", "xm", "it",
];

pub const DOCUMENT_PATH: &str = "Documents";
pub const DOCUMENT_EXTENSIONS: &[&str] = &[
    "txt", "pdf", "doc", "docx", "rtf", "odt", "pages", "xls", "xlsx", "xlsm", "csv", "ods",
    "numbers", "ppt", "pptx", "pps", "odp", "key", "md", "markdown", "json", "xml", "yaml", "yml",
    "toml", "conf", "ini", "cfg", "html", "htm", "css", "js", "ts", "rs", "py", "sh", "bat",
    "epub", "fb2", "mobi", "djvu", "djv", "zip", "rar", "7z", "tar", "gz", "bz2", "xz", "iso",
    "dmg",
];

pub const VIDEO_PATH: &str = "Videos";
pub const VIDEO_EXTENSIONS: &[&str] = &[
    "mp4", "mkv", "avi", "mov", "webm", "flv", "wmv", "asf", "mpeg", "mpg", "m4v", "ts", "mts",
    "m2ts", "vob", "ogv", "mxf", "prores", "dnxhr", "3gp", "3g2",
];

pub fn help_message() {
    println!(
        "\n\
            {BOLD_BLUE}No arguments!{DEFAULT_COLOR}\n\n\
            {BOLD_BLUE}Use:{DEFAULT_COLOR} {BOLD_GREEN}{CLI_COMMAND_NAME}{DEFAULT_COLOR} {BOLD}[DIRECTORY]{DEFAULT_COLOR}\n\n\
            {BOLD_BLUE}For example:{DEFAULT_COLOR}\n\
            {BOLD_GREEN}{CLI_COMMAND_NAME}{DEFAULT_COLOR} {BOLD}~{DEFAULT_COLOR}\n\
            {BOLD_GREEN}{CLI_COMMAND_NAME}{DEFAULT_COLOR} {BOLD}~/Downloads{DEFAULT_COLOR}\n"
    );
}

//CLI ui print colors
pub const DEFAULT_COLOR: &str = "\x1b[0m";
pub const BOLD_BLUE: &str = "\x1b[1;36m";
pub const BOLD_GREEN: &str = "\x1b[1;32m";
pub const BOLD: &str = "\x1b[1m";
