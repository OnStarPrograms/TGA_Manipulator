#[repr(C, packed)] // no safety bytes
#[derive(Debug, Copy, Clone)]
pub struct Header
{
    // Use a transmute funct (google it)
    pub id: u8,
    pub color_map: u8,
    pub image_type: u8,
    pub color_origin: u16,
    pub color_map_length: u16,
    pub color_map_depth: u8,
    pub x_origin: u16,
    pub y_origin: u16, 
    pub width: u16,
    pub height: u16,
    pub pixel_depth: u8,
    pub image_descriptor: u8
}