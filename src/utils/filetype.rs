const IMAGE_TYPES: [&str; 6] = ["jpg", "jpeg", "gif", "png", "svg", "webp"];
const VIDEO_TYPES: [&str; 2] = ["mp4", "webm"];

pub fn is_image(url: &str) -> bool {
  is_filetype(url, &IMAGE_TYPES)
}

pub fn is_video(url: &str) -> bool {
  is_filetype(url, &VIDEO_TYPES)
}

fn is_filetype(url: &str, exts: &[&str]) -> bool {
  url
    .split('?')
    .next()
    .and_then(|s| s.rsplit('.').next().map(str::to_lowercase))
    .is_some_and(|ext| exts.iter().any(|file_type| ext.ends_with(file_type)))
}
