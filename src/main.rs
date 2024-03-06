use std::env;
use std::fs;
// use std::path::Path;

fn main() -> std::io::Result<()>{
    //CLI Argument line
    let args: Vec<String> = env::args().collect();
    // let check = &args[1];
    // let file_path = String::from("poem.txt");
    // let root = Path::new("../../");
    // println!("Successfully changed working directory to {}!", root.display());
    // assert!(env::set_current_dir(&root).is_ok());
    // Fuck dude this was difficult just to read a txt file but I finally did it
    for element in args.iter() 
    {
        if element == "-R"
        {
            // dont add a "/" to the front of the path argument
            let contents = fs::read_to_string("input/poem.txt").expect("Should have been able to read the file");
            print!("{}", contents);
            println!("{element}");
 
            // Prints the current working dir
            let path = env::current_dir()?;
            print!("{}", path.display());
        }
    }

    // Everything is a-OK
    Ok(())
    // This is for testing reading the file
}
