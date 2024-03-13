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
    width: u16,
    height: u16,
}
// -R; -C; -F; -T; -M; -S; -N; -O; -Test;
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
            let vec = openfile(&args[i+1]);

            println!(" ");
            print!("{:p} : {:x}", &vec[0], &vec[0]);
        }


        //Writes image data into a struct
        if element == "-F"
        {
            let mut vec = openfile(&args[i+1]);

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
            // let v: Vec<u16> = vec![6, 6];
            let vec = openfile("input/text2.tga");

            // I copied this code from Stack Overflow
            // without understanding why this case is safe.
            let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
            assert!(head.is_empty(), "Data was not aligned");
            let mut my_struct = body[0];

            println!("{:?}", my_struct);
            my_struct.width = 6;
            my_struct.height = 6;
            let vec: Vec<u8> = vec![1,2,3,4,5,6, 2,3,4,5,6,7, 3,4,5,6,7,8, 4,5,6,7,8,9, 5,6,7,8,9,0, 6,7,8,9,0,1];
            for i in 0..my_struct.height
            {
                for j in 0..my_struct.width
                {
                    print!("{}", vec[(j+i*my_struct.width) as usize]);
                }
                println!("");
            }
            println!("");
            let newvec = header::flip180(vec);
            for i in 0..my_struct.height
            {
                for j in 0..my_struct.width
                {
                    print!("{}", newvec[(j+i*my_struct.width) as usize]);
                }
                println!("");
            }
        }



        //Multiplies 2 files
        if element == "-M"
        {
            let mut vec = openfile(&args[i+1]);
            let mut filter = openfile(&args[i+2]);



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
            let vec = header::multiply(vec, filter);
            println!("Done Multiplying \n New Vec 0: {}", vec[0]);
            println!("{:?}", my_struct);
        }



        //Subtracts two Files
        if element == "-S"
        {
            let mut vec = openfile(&args[i+1]);
            let mut filter = openfile(&args[i+2]);



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
            let vec = header::sub(vec, filter);
            println!("Done Subtracting \n New Vec 0: {}", vec[0]);
            println!("{:?}", my_struct);
        }



        //Screens (Divides) two images
        if element == "-D"
        {
            let mut vec = openfile(&args[i+1]);
            let mut filter = openfile(&args[i+2]);



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
            let vec = header::screen(vec, filter);
            println!("Done Screen \n New Vec 0: {}", vec[0]);
            println!("{:?}", my_struct);
        }

        if element == "-O"
        {
            let mut vec = openfile(&args[i+1]);
            let mut filter = openfile(&args[i+2]);



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
            let vec = header::overlay(vec, filter);
            println!("Done Overlay \n New Vec 0: {}", vec[0]);
            println!("{:?}", my_struct);
        }






        // Build a File
        if element == "-N"
        {
            let mut vec = openfile(&args[i+1]);



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
            write_file(vec, my_struct, "output/test.tga").unwrap();

        }



        //Tests two .tga Files completely
        if element == "-Test"
        {
            let vec = openfile(&args[i+1]);
            let filter = openfile(&args[i+2]);


            // let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
            // assert!(head.is_empty(), "Data was not aligned");
            // let my_struct = body[0];

            // println!("{:?}", my_struct);
            // println!(" ");
            // for _i in 0..18
            // {
            //     vec.remove(0);
            //     filter.remove(0);
            // }
            println!("original Vec 0: {}", vec[0]);
            println!("filter Vec 0: {}", filter[0]);
            //image data starts at "vec[18]"
            let vec = header::test(vec, filter);
            if vec.0 == 0 && vec.1 == 1 && vec.2 == 1
            {
                println!("Files are the same");
            }
            else 
            {
                println!("Error at");
                println!("index: {}", vec.0);
                println!("base: {:x}", vec.1);
                println!("top: {:x}", vec.2);
            }
            println!("");
            // println!("{:?}", my_struct);
        }
        i += 1;
    }
            // let path: String = String::from("input/car.tga");
            // // let mut path: String = String::new();
            // // io::stdin().read_line(&mut path).expect("failed to read line.");  
            // let bytes = fs::read(path).unwrap(); 
            // let mut vec: Vec<u8> = Vec::new();
            // for byte_pair in bytes.chunks_exact(1)
            // {
            //     let short = u8::from_le_bytes([byte_pair[0]]);
            //     vec.push(short);
            // }

            // let path: String = String::from("output/test.tga");
            // // let mut path: String = String::new();
            // // io::stdin().read_line(&mut path).expect("failed to read line.");  
            // let bytes = fs::read(path).unwrap(); 
            // let mut filter: Vec<u8> = Vec::new();
            // for byte_pair in bytes.chunks_exact(1)
            // {
            //     let short = u8::from_le_bytes([byte_pair[0]]);
            //     filter.push(short);
            // }


            // // let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
            // // assert!(head.is_empty(), "Data was not aligned");
            // // let my_struct = body[0];

            // // println!("{:?}", my_struct);
            // // println!(" ");
            // // for _i in 0..18
            // // {
            // //     vec.remove(0);
            // //     filter.remove(0);
            // // }
            // println!("original Vec 0: {}", vec[0]);
            // println!("filter Vec 0: {}", filter[0]);
            // //image data starts at "vec[18]"
            // let vec = test(vec, filter);
            // if vec.0 == 0 && vec.1 == 1 && vec.2 == 1
            // {
            //     println!("Files are the same");
            // }
            // else 
            // {
            //     println!("Error at");
            //     println!("index: {}", vec.0);
            //     println!("base: {:x}", vec.1);
            //     println!("top: {:x}", vec.2);
            // }
            // println!("");


    //Start of the actual code
    part1();
    // {
    //     //tests part1
    //     let vec = openfile("examples/EXAMPLE_part1.tga");
    //     let vec2 = openfile("output/part1.tga");

    //     let vec = header::test(vec, vec2);

    //     if vec.0 == 0 && vec.1 == 1 && vec.2 == 1
    //     {
    //         println!("Files are the same :D");
    //         println!("");
    //     }
    //     else 
    //     {
    //         println!("Error at");
    //         println!("index: {}", vec.0);
    //         println!("base: {:x}", vec.1);
    //         println!("top: {:x}", vec.2);
    //     }
    //     println!("");
    // }

    part2();
    // {
    //     //tests part1
    //     let vec = openfile("examples/EXAMPLE_part2.tga");
    //     let vec2 = openfile("output/part2.tga");

    //     let vec = header::test(vec, vec2);

    //     if vec.0 == 0 && vec.1 == 1 && vec.2 == 1
    //     {
    //         println!("Files are the same :D");
    //         println!("");
    //     }
    //     else 
    //     {
    //         println!("Error at");
    //         println!("index: {}", vec.0);
    //         println!("base: {:x}", vec.1);
    //         println!("top: {:x}", vec.2);
    //     }
    //     println!("");
    // }

    part3();
    // {
    //     //tests part1
    //     let vec = openfile("examples/EXAMPLE_part3.tga");
    //     let vec2 = openfile("output/part3.tga");

    //     let vec = header::test(vec, vec2);

    //     if vec.0 == 0 && vec.1 == 1 && vec.2 == 1
    //     {
    //         println!("Files are the same :D");
    //         println!("");
    //     }
    //     else 
    //     {
    //         println!("Error at");
    //         println!("index: {}", vec.0);
    //         println!("base: {:x}", vec.1);
    //         println!("top: {:x}", vec.2);
    //     }
    //     println!("");
    // }

    part4();
    // {
    //     //tests part1
    //     let vec = openfile("examples/EXAMPLE_part4.tga");
    //     let vec2 = openfile("output/part4.tga");

    //     let vec = header::test(vec, vec2);

    //     if vec.0 == 0 && vec.1 == 1 && vec.2 == 1
    //     {
    //         println!("Files are the same :D");
    //         println!("");
    //     }
    //     else 
    //     {
    //         println!("Error at");
    //         println!("index: {}", vec.0);
    //         println!("base: {:x}", vec.1);
    //         println!("top: {:x}", vec.2);
    //     }
    //     println!("");
    // }

    part5();
    // {
    //     //tests part1
    //     let vec = openfile("examples/EXAMPLE_part5.tga");
    //     let vec2 = openfile("output/part5.tga");

    //     let vec = header::test(vec, vec2);

    //     if vec.0 == 0 && vec.1 == 1 && vec.2 == 1
    //     {
    //         println!("Files are the same :D");
    //         println!("");
    //     }
    //     else 
    //     {
    //         println!("Error at");
    //         println!("index: {}", vec.0);
    //         println!("base: {:x}", vec.1);
    //         println!("top: {:x}", vec.2);
    //     }
    //     println!("");
    // }

    part6();
    // {
    //     //tests part1
    //     let vec = openfile("examples/EXAMPLE_part6.tga");
    //     let vec2 = openfile("output/part6.tga");

    //     let vec = header::test(vec, vec2);

    //     if vec.0 == 0 && vec.1 == 1 && vec.2 == 1
    //     {
    //         println!("Files are the same :D");
    //         println!("");
    //     }
    //     else 
    //     {
    //         println!("Error at");
    //         println!("index: {}", vec.0);
    //         println!("base: {:x}", vec.1);
    //         println!("top: {:x}", vec.2);
    //     }
    //     println!("");
    // }

    part7();
    // {
    //     //tests part1
    //     let vec = openfile("examples/EXAMPLE_part7.tga");
    //     let vec2 = openfile("output/part7.tga");

    //     let vec = header::test(vec, vec2);

    //     if vec.0 == 0 && vec.1 == 1 && vec.2 == 1
    //     {
    //         println!("Files are the same :D");
    //         println!("");
    //     }
    //     else 
    //     {
    //         println!("Error at");
    //         println!("index: {}", vec.0);
    //         println!("base: {:x}", vec.1);
    //         println!("top: {:x}", vec.2);
    //     }
    //     println!("");
    // }

    part8();
    // {
    //     //tests part1
    //     let vec = openfile("examples/EXAMPLE_part8_g.tga");
    //     let vec2 = openfile("output/part8_g.tga");

    //     let vec = header::test(vec, vec2);

    //     if vec.0 == 0 && vec.1 == 1 && vec.2 == 1
    //     {
    //         println!("Files are the same :D");
    //         // println!("");
    //     }
    //     else 
    //     {
    //         println!("Error at");
    //         println!("index: {}", vec.0);
    //         println!("base: {:x}", vec.1);
    //         println!("top: {:x}", vec.2);
    //     }
    //     // println!("");
    // }

    // {
    //     //tests part1
    //     let vec = openfile("examples/EXAMPLE_part8_b.tga");
    //     let vec2 = openfile("output/part8_b.tga");

    //     let vec = header::test(vec, vec2);

    //     if vec.0 == 0 && vec.1 == 1 && vec.2 == 1
    //     {
    //         println!("Files are the same :D");
    //         // println!("");
    //     }
    //     else 
    //     {
    //         println!("Error at");
    //         println!("index: {}", vec.0);
    //         println!("base: {:x}", vec.1);
    //         println!("top: {:x}", vec.2);
    //     }
    //     // println!("");
    // }

    // {
    //     //tests part1
    //     let vec = openfile("examples/EXAMPLE_part8_r.tga");
    //     let vec2 = openfile("output/part8_r.tga");

    //     let vec = header::test(vec, vec2);

    //     if vec.0 == 0 && vec.1 == 1 && vec.2 == 1
    //     {
    //         println!("Files are the same :D");
    //         println!("");
    //     }
    //     else 
    //     {
    //         println!("Error at");
    //         println!("index: {}", vec.0);
    //         println!("base: {:x}", vec.1);
    //         println!("top: {:x}", vec.2);
    //     }
    //     println!("");
    // }

    part9();
    // {
    //     //tests part1
    //     let vec = openfile("examples/EXAMPLE_part9.tga");
    //     let vec2 = openfile("output/part9.tga");

    //     let vec = header::test(vec, vec2);

    //     if vec.0 == 0 && vec.1 == 1 && vec.2 == 1
    //     {
    //         println!("Files are the same :D");
    //         println!("");
    //     }
    //     else 
    //     {
    //         println!("Error at");
    //         println!("index: {}", vec.0);
    //         println!("base: {:x}", vec.1);
    //         println!("top: {:x}", vec.2);
    //     }
    //     println!("");
    // }

    //Error
    part10();
    // {
    //     //tests part1
    //     let vec = openfile("examples/EXAMPLE_part10.tga");
    //     let vec2 = openfile("output/part10.tga");

    //     let vec = header::test(vec, vec2);

    //     if vec.0 == 0 && vec.1 == 1 && vec.2 == 1
    //     {
    //         println!("Files are the same :D");
    //         println!("");
    //     }
    //     else 
    //     {
    //         println!("Error at");
    //         println!("index: {}", vec.0);
    //         println!("base: {:x}", vec.1);
    //         println!("top: {:x}", vec.2);
    //     }
    //     println!("");
    // }


    // Everything is a-OK
    Ok(())
    // This is for testing reading the file
}




/*
*
*
*
* ////////////////////////////////////////////////////////////////
*
*
*/

fn part1()
{
    let mut vec = openfile("input/layer1.tga");
    let mut vec2 = openfile("input/pattern1.tga");


    let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
    assert!(head.is_empty(), "Data was not aligned");
    let my_struct = body[0];

    for _i in 0..18
    {
        vec.remove(0);
        vec2.remove(0);
    }

    let vec = header::multiply(vec, vec2);
    //image data starts at "vec[18]"

    // let path = String::from("output/testoutput.tga");
    // let contents = String::from(stringify!(my_struct.id));
    
    // Creates a new File
    write_file(vec, my_struct, "output/part1.tga").unwrap();
    println!("--Done Multiplying!");

}

/*
*
*
*
* ////////////////////////////////////////////////////////////////
*
*
*/

fn part2()
{
    let mut vec2 = openfile("input/layer2.tga");
    let mut vec = openfile("input/car.tga");


    let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
    assert!(head.is_empty(), "Data was not aligned");
    let my_struct = body[0];

    for _i in 0..18
    {
        vec.remove(0);
        vec2.remove(0);
    }

    let vec = header::sub(vec, vec2);
    //image data starts at "vec[18]"

    // let path = String::from("output/testoutput.tga");
    // let contents = String::from(stringify!(my_struct.id));
    
    // Creates a new File
    write_file(vec, my_struct, "output/part2.tga").unwrap();
    println!("--Done Subtracting!");

}

/*
*
*
*
* ////////////////////////////////////////////////////////////////
*
*
*/

fn part3()
{
    let mut vec = openfile("input/layer1.tga");
    let mut vec2 = openfile("input/pattern2.tga");


    let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
    assert!(head.is_empty(), "Data was not aligned");
    let my_struct = body[0];

    for _i in 0..18
    {
        vec.remove(0);
        vec2.remove(0);
    }

    let vec = header::multiply(vec, vec2);
    let mut vec2 = openfile("input/text.tga");
    for _i in 0..18
    {
        vec2.remove(0);
    }
    let vec = header::screen(vec, vec2);

    //image data starts at "vec[18]"

    // let path = String::from("output/testoutput.tga");
    // let contents = String::from(stringify!(my_struct.id));
    
    // Creates a new File
    write_file(vec, my_struct, "output/part3.tga").unwrap();
    println!("--Done Part3!");

}

/*
*
*
*
* ////////////////////////////////////////////////////////////////
*
*
*/

fn part4()
{
    let mut vec = openfile("input/layer2.tga");
    let mut vec2 = openfile("input/circles.tga");


    let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
    assert!(head.is_empty(), "Data was not aligned");
    let my_struct = body[0];

    for _i in 0..18
    {
        vec.remove(0);
        vec2.remove(0);
    }

    let vec = header::multiply(vec, vec2);
    let mut vec2 = openfile("input/pattern2.tga");
    for _i in 0..18
    {
        vec2.remove(0);
    }
    let vec = header::sub(vec, vec2);

    //image data starts at "vec[18]"

    // let path = String::from("output/testoutput.tga");
    // let contents = String::from(stringify!(my_struct.id));
    
    // Creates a new File
    write_file(vec, my_struct, "output/part4.tga").unwrap();
    println!("--Done Part4!");

}

/*
*
*
*
* ////////////////////////////////////////////////////////////////
*
*
*/

fn part5()
{
    let mut vec2 = openfile("input/layer1.tga");
    let mut vec = openfile("input/pattern1.tga");


    let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
    assert!(head.is_empty(), "Data was not aligned");
    let my_struct = body[0];

    for _i in 0..18
    {
        vec.remove(0);
        vec2.remove(0);
    }

    let vec = header::overlay(vec, vec2);
    // let mut vec2 = openfile("input/pattern2.tga");
    // for _i in 0..18
    // {
    //     vec2.remove(0);
    // }
    // let vec = header::sub(vec, vec2);

    //image data starts at "vec[18]"

    // let path = String::from("output/testoutput.tga");
    // let contents = String::from(stringify!(my_struct.id));
    
    // Creates a new File
    write_file(vec, my_struct, "output/part5.tga").unwrap();
    println!("--Done Part5!");

}

/*
*
*
*
* ////////////////////////////////////////////////////////////////
*
*
*/

fn part6()
{
    let mut vec = openfile("input/car.tga");
    // let mut vec = openfile("input/pattern1.tga");


    let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
    assert!(head.is_empty(), "Data was not aligned");
    let my_struct = body[0];

    for _i in 0..18
    {
        vec.remove(0);
    }

    let vec = header::add(vec, 200, 0, 0);
    // let mut vec2 = openfile("input/pattern2.tga");
    // for _i in 0..18
    // {
    //     vec2.remove(0);
    // }
    // let vec = header::sub(vec, vec2);

    //image data starts at "vec[18]"

    // let path = String::from("output/testoutput.tga");
    // let contents = String::from(stringify!(my_struct.id));
    
    // Creates a new File
    write_file(vec, my_struct, "output/part6.tga").unwrap();
    println!("--Done Part6!");

}

/*
*
*
*
* ////////////////////////////////////////////////////////////////
*
*
*/

fn part7()
{
    let mut vec = openfile("input/car.tga");
    // let mut vec = openfile("input/pattern1.tga");


    let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
    assert!(head.is_empty(), "Data was not aligned");
    let my_struct = body[0];

    for _i in 0..18
    {
        vec.remove(0);
    }

    let vec = header::scale(vec, 1, 0, 4);
    // let mut vec2 = openfile("input/pattern2.tga");
    // for _i in 0..18
    // {
    //     vec2.remove(0);
    // }
    // let vec = header::sub(vec, vec2);

    //image data starts at "vec[18]"

    // let path = String::from("output/testoutput.tga");
    // let contents = String::from(stringify!(my_struct.id));
    
    // Creates a new File
    write_file(vec, my_struct, "output/part7.tga").unwrap();
    println!("--Done Part7!");

}

/*
*
*
*
* ////////////////////////////////////////////////////////////////
*
*
*/

fn part8()
{
    let mut vec = openfile("input/car.tga");
    // let mut vec = openfile("input/pattern1.tga");


    let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
    assert!(head.is_empty(), "Data was not aligned");
    let my_struct = body[0];

    for _i in 0..18
    {
        vec.remove(0);
    }

    let mut blue: Vec<u8> = vec![];
    let mut green: Vec<u8> = vec![];
    let mut red: Vec<u8> = vec![];
    for chunk in vec.chunks_exact(3)
    {
        blue.push(chunk[0]);
        blue.push(chunk[0]);
        blue.push(chunk[0]);

        green.push(chunk[1]);
        green.push(chunk[1]);
        green.push(chunk[1]);

        red.push(chunk[2]);
        red.push(chunk[2]);
        red.push(chunk[2]);
    }
    // let mut vec2 = openfile("input/pattern2.tga");
    // for _i in 0..18
    // {
    //     vec2.remove(0);
    // }
    // let vec = header::sub(vec, vec2);

    //image data starts at "vec[18]"

    // let path = String::from("output/testoutput.tga");
    // let contents = String::from(stringify!(my_struct.id));
    
    // Creates a new File
    write_file(blue, my_struct, "output/part8_b.tga").unwrap();
    write_file(green, my_struct, "output/part8_g.tga").unwrap();
    write_file(red, my_struct, "output/part8_r.tga").unwrap();
    println!("--Done Part8!");

}

/*
*
*
*
* ////////////////////////////////////////////////////////////////
*
*
*/

fn part9()
{
    let mut green = openfile("input/layer_green.tga");
    let mut blue = openfile("input/layer_blue.tga");
    let mut red = openfile("input/layer_red.tga");
    // let mut vec = openfile("input/pattern1.tga");


    let (head, body, _tail) = unsafe { green.align_to::<Header>() };
    assert!(head.is_empty(), "Data was not aligned");
    let my_struct = body[0];

    for _i in 0..18
    {
        green.remove(0);
        red.remove(0);
        blue.remove(0);
    }

    let mut vec = vec![];

    let mut j = 0;
    for i in green.chunks_exact(3)
    {
        vec.push(blue[j]);
        vec.push(i[0]);
        vec.push(red[j]);
        j += 3;
    }
    // let mut vec2 = openfile("input/pattern2.tga");
    // for _i in 0..18
    // {
    //     vec2.remove(0);
    // }
    // let vec = header::sub(vec, vec2);

    //image data starts at "vec[18]"

    // let path = String::from("output/testoutput.tga");
    // let contents = String::from(stringify!(my_struct.id));
    
    // Creates a new File
    write_file(vec, my_struct, "output/part9.tga").unwrap();
    println!("--Done Part9!");

}

/*
*
*
*
* ////////////////////////////////////////////////////////////////
*
*
*/

fn part10()
{
    let mut vec = openfile("input/text2.tga");


    let (head, body, _tail) = unsafe { vec.align_to::<Header>() };
    assert!(head.is_empty(), "Data was not aligned");
    let my_struct = body[0];

    for _i in 0..18
    {
        vec.remove(0);
    }

    let vecer = header::flip180(vec);

    //image data starts at "vec[18]"

    // let path = String::from("output/testoutput.tga");
    // let contents = String::from(stringify!(my_struct.id));
    
    // Creates a new File
    write_file(vecer, my_struct, "output/part10.tga").unwrap();
    println!("--Done Part10!");

}

/*
*
*
*
* ////////////////////////////////////////////////////////////////
*
*
*/

fn write_file(vec: Vec<u8>, my_struct: Header, output: &str) -> std::io::Result<()>
{
    //image data starts at "vec[18]"
    println!("Vec 0: {}", vec[0]);
    println!("{:?}", my_struct);

    // let path = String::from("output/testoutput.tga");
    // let contents = String::from(stringify!(my_struct.id));
            
    // Creates a new File
    let mut write_file = File::create(output).unwrap();
            
    // let mut file = File::open("output/test.tga")?;

    // Used to write the header struct

    write_file.write_all(&[my_struct.id, my_struct.color_map, my_struct.image_type])?;
    write_file.write_all(&my_struct.color_origin.to_le_bytes())?;
    write_file.write_all(&my_struct.color_map_length.to_le_bytes())?;
    write_file.write_all(&[my_struct.color_map_depth])?;
    write_file.write_all(&my_struct.x_origin.to_be_bytes())?;
    write_file.write_all(&my_struct.y_origin.to_be_bytes())?;
    write_file.write_all(&my_struct.width.to_le_bytes())?;
    write_file.write_all(&my_struct.height.to_le_bytes())?;
    write_file.write_all(&[my_struct.pixel_depth, my_struct.image_descriptor])?;
    //my_struct.color_origin.to_be_bytes()


/*/////////////////////////////////////////////////////////////////// */
    // used to write in the image data
    let mut writer = BufWriter::new(&write_file);

    // //Writes the actual Data
    let io_buf = IoSlice::new(&vec);

    writer.write_vectored(&[io_buf]).unwrap();

    Ok(())
}



fn openfile(pather: &str) -> Vec<u8>
{
    let path: String = String::from(pather);
    // let mut path: String = String::new();
    // io::stdin().read_line(&mut path).expect("failed to read line.");  
    let bytes = fs::read(path).unwrap(); 
    let mut vec: Vec<u8> = Vec::new();
    for byte_pair in bytes.chunks_exact(1)
    {
        let short = u8::from_le_bytes([byte_pair[0]]);
        vec.push(short);
    }
    return vec;
}