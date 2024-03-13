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

pub fn test(base: Vec<u8>, top: Vec<u8>) -> (usize, u8, u8)
{
    let mut k = 0;
    for i in &base
    {
        let _mi = *i;
        let _mj = top[k];
        if top[k] != *i
        {
            return (k, *i, top[k]);
        }
        k += 1;
    }
    return (0, 1, 1);
}




pub fn multiply(mut base: Vec<u8>, top: Vec<u8>) -> Vec<u8>
{
    let mut i = 0;
    for byte_pair in top.chunks_exact(3)
    {
        let redf = byte_pair[0];
        let redl1 = base[i];
        let greenf = byte_pair[1];
        let greenl1 = base[i+1];
        let bluef = byte_pair[2];
        let bluel1 = base[i+2];
        let mut check255: f32;

        check255 = (redl1 as f32 / 255.) * (redf as f32 / 255.);
        base[i] = (check255 * 255. + 0.5) as u8;

        check255 = (greenl1 as f32 / 255.) * (greenf as f32 / 255.);
        base[i+1] = (check255 * 255. + 0.5) as u8;

        check255 = (bluel1 as f32 / 255.) * (bluef as f32 / 255.);
        base[i+2] = (check255 * 255. + 0.5) as u8;

        i+=3;
    }

    return base;
}

pub fn screen(mut base: Vec<u8>, top: Vec<u8>) -> Vec<u8>
{
    let mut i = 0;
    for byte_pair in top.chunks_exact(3)
    {
        let redf = byte_pair[0];
        let redl1 = base[i];
        let greenf = byte_pair[1];
        let greenl1 = base[i+1];
        let bluef = byte_pair[2];
        let bluel1 = base[i+2];
        let mut check255: f32;

        check255 = 1.0 - (1.0 - (redl1 as f32 / 255.)) * (1.0 - (redf as f32 / 255.));
        base[i] = (check255 * 255. + 0.5) as u8;

        check255 = 1.0 - (1.0 - (greenl1 as f32 / 255.)) * (1.0 - (greenf as f32 / 255.));
        base[i+1] = (check255 * 255. + 0.5) as u8;

        check255 = 1.0 - (1.0 - (bluel1 as f32 / 255.)) * (1.0 - (bluef as f32 / 255.));
        base[i+2] = (check255 * 255. + 0.5) as u8;

        i+=3;
    }

    return base;
}

pub fn add(mut base: Vec<u8>, green: u8, blue: u8, red: u8) -> Vec<u8>
{
    let mut i = 0;
    
    for _j in 0..base.len()/3
    {
        let nred = base[i+2];

        if nred as u16 + red as u16 <= 255
        {
            base[i+2] = nred + red
        }
        else 
        {
            base[i+2] = 255;
        }

        let ngreen = base[i+1];

        if ngreen as u16 + green as u16 <= 255
        {
            base[i+1] = ngreen + green
        }
        else 
        {
            base[i+1] = 255;
        }

        let nblue = base[i];

        if nblue as u16 + blue as u16 <= 255
        {
            base[i] = nblue + blue
        }
        else 
        {
            base[i] = 255;
        }

        i+=3;
    }

    return base;
}

pub fn sub(mut base: Vec<u8>, top: Vec<u8>) -> Vec<u8>
{
    let mut i = 0;
    for byte_pair in top.chunks_exact(3)
    {
        let redf = byte_pair[0];
        let redl1 = base[i];
        let greenf = byte_pair[1];
        let greenl1 = base[i+1];
        let bluef = byte_pair[2];
        let bluel1 = base[i+2];
        let mut check255: i32;

        check255 = (redl1 as i32) - (redf as i32);
        if check255 < 0
        {
            base[i] = 0;
        }
        else
        {
            base[i] = (check255) as u8;
        }

        check255 = (greenl1 as i32) - (greenf as i32);
        if check255 < 0
        {
            base[i+1] = 0;
        }
        else
        {
            base[i+1] = (check255) as u8;
        }

        check255 = (bluel1 as i32) - (bluef as i32);
        if check255 < 0
        {
            base[i+2] = 0;
        }
        else
        {
            base[i+2] = (check255) as u8;
        }

        i+=3;
    }
    return base;
}

pub fn overlay(mut base: Vec<u8>, top: Vec<u8>) -> Vec<u8>
{
    let mut i = 0;
    for overlayer in top.chunks_exact(3)
    {
        let blueb = base[i];
        let bluef = overlayer[0];
        let greenb = base[i+1];
        let greenf = overlayer[1];
        let redb = base[i+2];
        let redf = overlayer[2];

        let mut check255: f32;

        if (blueb as f32 / 255.) > 0.5
        {
            check255 = 1.0 - 2. * (1.0 - (blueb as f32 / 255.)) * (1.0 - (bluef as f32 / 255.));
        }
        else
        {
            check255 = 2. * (blueb as f32 / 255.) * (bluef as f32 / 255.);
        }
        base[i] = (check255 * 255. + 0.5) as u8;



        if (greenb as f32 / 255.) > 0.5
        {
            check255 = 1.0 - 2.0 * (1.0 - (greenb as f32 / 255.)) * (1.0 - (greenf as f32 / 255.));
        }
        else
        {
            check255 = 2. * (greenb as f32 / 255.) * (greenf as f32 / 255.);
        }
        base[i+1] = (check255 * 255. + 0.5) as u8;



        if (redb as f32 / 255.) > 0.5
        {
            check255 = 1.0 - 2.0 * (1.0 - (redb as f32 / 255.)) * (1.0 - (redf as f32 / 255.));
        }
        else
        {
            check255 = 2. * (redb as f32 / 255.) * (redf as f32 / 255.);
        }
        base[i+2] = (check255 * 255. + 0.5) as u8;

        i+=3;
    }
    return base;
}

pub fn flip180(mut base: Vec<u8>, my_struct: Header) -> Vec<u8>
{
    let mut bottom;
    let mut top;
    let mut safe;
    let fheight = (my_struct.height) as usize;
    let width = (my_struct.width*3) as usize;

    for i in 0..((fheight/2)*width)
    {
        bottom = fheight*width-(i+1);
        top = i;

        safe = base[bottom];
        base[bottom] = base[top];
        base[top] = safe;
    }
    // let mut j: usize = 0;
    // for i in 0..((fheight/2)*width)
    // {
    //     if i % width == 0
    //     {
    //         j+=1;
    //     }
    //     safe = base[width-(i%width+1) + width*j];
    //     base[width-(i%width+1) + width*j] = base[(i%width) + width*j];
    //     base[(i%width) + width*j] = safe;
    // }
    return base;
}