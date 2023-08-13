
pub struct Image {
  pub width: u32,
  pub height: u32,
  pub pitch: u32,
  //pixel format
  pub pixels: std::vec::Vec<u8>,
}

