#[repr(C, packed)] // no safety bytes
#[derive(Debug, Copy, Clone)]
pub struct Header
{
    // Use a transmute funct (google it)
    id: u8,
    color_map: u8,
    image_type: u8,
    color_origin: u16,
    color_map_length: u16,
    color_map_depth: u8,
    x_origin: u16,
    y_origin: u16, 
    width: u16,
    height: u16,
    pixel_depth: u8,
    image_descriptor: u8
}