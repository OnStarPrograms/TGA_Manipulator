mod header; //used to import a local file

use std::env;

use std::fs;
use std::fs::File;
use std::io::{BufWriter, IoSlice, Write};
use header::Header;
// use std::io;
// use std::path::Path;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct MyStruct {
    foo: u8,
    bar: u8,
    car: u8,
}
// -R; -C; -F; -T; -M; -S; -N;
fn main() -> std::io::Result<()>{
    //CLI Argument line
    let args: Vec<String> = env::args().collect();
    // let check = &args[1];
    // let file_path = String::from("poem.txt");
    // let root = Path::new("../../");
    // println!("Successfully changed working directory to {}!", root.display());
    // assert!(env::set_current_dir(&root).is_ok());
    let mut i = 0;
    for element in args.iter() 
    {
        // pulls the poem and the car.tga and prints them
        if element == "-R"
        {
            
            // dont add a "/" to the front of the path argument
            let path: String = String::from("input/car.tga"); // remind: mut it

            let bytes = fs::read(path).unwrap();
            let contents = fs::read_to_string("input/poem.txt").expect("Should have been able to read the file");
            print!("{}", contents);
            println!("{element}");
            let mut vec: Vec<u8> = Vec::new();
            for byte_pair in bytes.chunks_exact(1)
            {
                let short = u8::from_le_bytes([byte_pair[0]]);
                vec.push(short);
                print!("{:x} ", short);
            }
            println!(" ");
            // this is the data
            println!("{}", vec[0]);
            // Prints the current working dir
            let path = env::current_dir()?;
            print!("{}", path.display());
        }
        // plan to Use to call a change function format CLI {cargo run -- -C input/file1 input/file2 manipulator}
        // need to build this pls
        // currently can only do (cargo run -- -C input/file1)
        //Pulls and prints one image data plus pointer
        if element == "-C"
        {
            let path: String = String::from(&args[i+1]);
            // let mut path: String = String::new();
            // io::stdin().read_line(&mut path).expect("failed to read line.");  
            let bytes = fs::read(path).unwrap();
            let mut vec: Vec<u8> = Vec::new();
            for byte_pair in bytes.chunks_exact(1)
            {
                let short = u8::from_le_bytes([byte_pair[0]]);
                vec.push(short);
                print!("{:x} ", short);
            }
            println!(" ");
            print!("{:p} : {:x}", &vec[0], &vec[0]);
        }


        //Writes image data into a struct
        if element == "-F"
        {
            let path: String = String::from(&args[i+1]);
            // let mut path: String = String::new();
            // io::stdin().read_line(&mut path).expect("failed to read line.");  
            let bytes = fs::read(path).unwrap(); 
            let mut vec: Vec<u8> = Vec::new();
            for byte_pair in bytes.chunks_exact(1)
            {
                let short = u8::from_le_bytes([byte_pair[0]]);
                vec.push(short);
                print!("{:x} ", short);
            }
            println!("");

            // align the memory pointer into the structure of the object
            /*
            

            If you don't want to copy the data to the struct 
            but instead leave it in place, 
            you can use slice::align_to. This "Moves" and creates a 
            &MyStruct instead


            Here, it's safe to use align_to to transmute some bytes
            to MyStruct because we've used repr(C, packed) and all 
            of the types in MyStruct can be any arbitrary bytes.
            

            Stck overflow link
            {
            https://stackoverflow.com/questions/42499049/how-to-transmute-a-u8-buffer-to-struct-in-rust
            }
            
             */
            let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
            assert!(head.is_empty(), "Data was not aligned");
            let my_struct = &body[0];

            println!("{:?}", my_struct);
            println!(" ");
            for _i in 0..18
            {
                vec.remove(0);
            }
    
            //image data starts at "vec[18]"
            print!("{:p} : {:x}", &vec[0], &vec[0]);
        }


        //Test Writing data into a struct
        if element == "-T"
        {
            let v: Vec<u8> = vec![5, 2, 3];

            // I copied this code from Stack Overflow
            // without understanding why this case is safe.
            let (head, body, _tail) = unsafe { v.align_to::<MyStruct>() };
            assert!(head.is_empty(), "Data was not aligned");
            let my_struct = &body[0];

            println!("{:?}", my_struct);
            println!("{:?}", head);
        }



        //Multiplies 2 files
        if element == "-M"
        {
            let path: String = String::from(&args[i+1]);
            // let mut path: String = String::new();
            // io::stdin().read_line(&mut path).expect("failed to read line.");  
            let bytes = fs::read(path).unwrap(); 
            let mut vec: Vec<u8> = Vec::new();
            for byte_pair in bytes.chunks_exact(1)
            {
                let short = u8::from_le_bytes([byte_pair[0]]);
                vec.push(short);
            }

            let path: String = String::from(&args[i+2]);
            // let mut path: String = String::new();
            // io::stdin().read_line(&mut path).expect("failed to read line.");  
            let bytes = fs::read(path).unwrap(); 
            let mut filter: Vec<u8> = Vec::new();
            for byte_pair in bytes.chunks_exact(1)
            {
                let short = u8::from_le_bytes([byte_pair[0]]);
                filter.push(short);
            }


            let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
            assert!(head.is_empty(), "Data was not aligned");
            let my_struct = body[0];

            println!("{:?}", my_struct);
            println!(" ");
            for _i in 0..18
            {
                vec.remove(0);
                filter.remove(0);
            }
            println!("original Vec 0: {}", vec[0]);
            println!("filter Vec 0: {}", filter[0]);
            //image data starts at "vec[18]"
            let vec = multiply(vec, filter);
            println!("Done Multiplying \n New Vec 0: {}", vec[0]);
            println!("{:?}", my_struct);
        }



        //Subtracts two Files
        if element == "-S"
        {
            let path: String = String::from(&args[i+1]);
            // let mut path: String = String::new();
            // io::stdin().read_line(&mut path).expect("failed to read line.");  
            let bytes = fs::read(path).unwrap(); 
            let mut vec: Vec<u8> = Vec::new();
            for byte_pair in bytes.chunks_exact(1)
            {
                let short = u8::from_le_bytes([byte_pair[0]]);
                vec.push(short);
            }

            let path: String = String::from(&args[i+2]);
            // let mut path: String = String::new();
            // io::stdin().read_line(&mut path).expect("failed to read line.");  
            let bytes = fs::read(path).unwrap(); 
            let mut filter: Vec<u8> = Vec::new();
            for byte_pair in bytes.chunks_exact(1)
            {
                let short = u8::from_le_bytes([byte_pair[0]]);
                filter.push(short);
            }


            let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
            assert!(head.is_empty(), "Data was not aligned");
            let my_struct = body[0];

            println!("{:?}", my_struct);
            println!(" ");
            for _i in 0..18
            {
                vec.remove(0);
                filter.remove(0);
            }
            println!("original Vec 0: {}", vec[0]);
            println!("filter Vec 0: {}", filter[0]);
            //image data starts at "vec[18]"
            let vec = sub(vec, filter);
            println!("Done Subtracting \n New Vec 0: {}", vec[0]);
            println!("{:?}", my_struct);
        }



        //Screens (Divides) two images
        if element == "-D"
        {
            let path: String = String::from(&args[i+1]);
            // let mut path: String = String::new();
            // io::stdin().read_line(&mut path).expect("failed to read line.");  
            let bytes = fs::read(path).unwrap(); 
            let mut vec: Vec<u8> = Vec::new();
            for byte_pair in bytes.chunks_exact(1)
            {
                let short = u8::from_le_bytes([byte_pair[0]]);
                vec.push(short);
            }

            let path: String = String::from(&args[i+2]);
            // let mut path: String = String::new();
            // io::stdin().read_line(&mut path).expect("failed to read line.");  
            let bytes = fs::read(path).unwrap(); 
            let mut filter: Vec<u8> = Vec::new();
            for byte_pair in bytes.chunks_exact(1)
            {
                let short = u8::from_le_bytes([byte_pair[0]]);
                filter.push(short);
            }


            let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
            assert!(head.is_empty(), "Data was not aligned");
            let my_struct = body[0];

            println!("{:?}", my_struct);
            println!(" ");
            for _i in 0..18
            {
                vec.remove(0);
                filter.remove(0);
            }
            println!("original Vec 0: {}", vec[0]);
            println!("filter Vec 0: {}", filter[0]);
            //image data starts at "vec[18]"
            let vec = screen(vec, filter);
            println!("Done Screen \n New Vec 0: {}", vec[0]);
            println!("{:?}", my_struct);
        }





        // Build a File
        if element == "-N"
        {
            let path: String = String::from(&args[i+1]);
            // let mut path: String = String::new();
            // io::stdin().read_line(&mut path).expect("failed to read line.");  
            let bytes = fs::read(path).unwrap(); 
            let mut vec: Vec<u8> = Vec::new();
            for byte_pair in bytes.chunks_exact(1)
            {
                let short = u8::from_le_bytes([byte_pair[0]]);
                vec.push(short);
            }


            let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
            assert!(head.is_empty(), "Data was not aligned");
            let my_struct = body[0];

            println!("{:?}", my_struct);
            println!(" ");
            let mut header_redundancy: Vec<u8> = Vec::new();
            for _i in 0..18
            {
                header_redundancy.push(vec[0]);
                vec.remove(0);
            }
            //image data starts at "vec[18]"
            println!("Vec 0: {}", vec[0]);
            println!("{:?}", my_struct);

            // let path = String::from("output/testoutput.tga");
            // let contents = String::from(stringify!(my_struct.id));
            
            // Creates a new File
            let mut write_file = File::create("output/test.tga").unwrap();
            
            // let mut file = File::open("output/test.tga")?;

            // Used to write the header struct

	        write_file.write_all(&[my_struct.id, my_struct.color_map, my_struct.image_type])?;
            write_file.write_all(&my_struct.color_origin.to_be_bytes())?;
            write_file.write_all(&my_struct.color_map_length.to_be_bytes())?;
            write_file.write_all(&[my_struct.color_map_depth])?;
            write_file.write_all(&my_struct.x_origin.to_be_bytes())?;
            write_file.write_all(&my_struct.y_origin.to_be_bytes())?;
            write_file.write_all(&my_struct.width.to_be_bytes())?;
            write_file.write_all(&my_struct.height.to_be_bytes())?;
            write_file.write_all(&[my_struct.pixel_depth, my_struct.image_descriptor])?;
            //my_struct.color_origin.to_be_bytes()


        /*/////////////////////////////////////////////////////////////////// */
            // used to write in the image data
            let mut writer = BufWriter::new(&write_file);

            // //Writes the actual Data
            let io_buf = IoSlice::new(&vec);

            writer.write_vectored(&[io_buf]).unwrap();

        /*////////////////////////////////////////////////////////////////// */
            
            //Writes the Header data in a Vec into the file *Need to figure out how to write a struct into the file*
            // let io_buf = IoSlice::new(&my_struct.id.to_be_bytes());

            // writer.write_vectored(&[io_buf]).unwrap();


            //Given the Header Struct "my_struct" and image data "vec";
            //Write it into a new file and save it to the output folder
            /* \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ \/ */


        }
        i += 1;
    }

    // Everything is a-OK
    Ok(())
    // This is for testing reading the file
}







fn multiply(mut base: Vec<u8>, top: Vec<u8>) -> Vec<u8>
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

fn screen(mut base: Vec<u8>, top: Vec<u8>) -> Vec<u8>
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

fn sub(mut base: Vec<u8>, top: Vec<u8>) -> Vec<u8>
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

fn overlay(mut base: Vec<u8>, top: Vec<u8>) -> Vec<u8>
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