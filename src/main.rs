use structopt::StructOpt;
use clap::Clap;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::Write;
use termimad::*;
use std::error::Error;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    #[structopt(parse(from_os_str))]
    name: PathBuf,

    #[structopt(short, long)]
    input: Option<String>,

    #[structopt(short, long)]
    output : Option<String>,

    #[structopt(short, long)]
    advanced: bool,

    #[structopt(long)]
    git: bool,
}

const NUM_STEPS: usize = 7;

fn main() {
    // Read arguments
    let opt = Opt::from_args();

    let mut skin = MadSkin::default();
    skin.italic.set_fg(rgb(139, 0, 139));
    skin.bold.set_fg(rgb(50, 205, 50));
    let mut process_step: usize = 1;

    // Create main directory
    match fs::create_dir(&opt.name) {
        Ok(_) => skin.print_inline(&format!("**[{}/{}]** Successfully created main folder: *{}*\n", process_step, NUM_STEPS, (&opt.name).display())),
        Err(err) => {
            skin.print_inline(&format!("**[{}/{}]** Failed to create main folder: *{}*\n", process_step, NUM_STEPS, err));
        },
    }

    process_step += 1;

    let mut main_content = String::from("#include<iostream>");
    let mut makefile_content = String::from(
"# the compiler
CC = g++

# compiler flags:
#  -Wall turns on most, but not all, compiler warnings
CFLAGS  = -Wall

# the build target executable:
TARGET = main");
    let mut io_content = String::new();

    let mut is_input_file = false;
    let mut is_output_file = false;

    match opt.input.clone() {
        Some(input_name) => {
            is_input_file = true;

            let mut path = opt.name.clone();
            path.push(input_name.clone());
            match File::create(&path) {
                Ok(_) => (),
                Err(why) => panic!("Couldn't create {}: {}", path.display(), why),
            };

            makefile_content = format!("{}\n\n# I/O files\nINPUT = {}", makefile_content, input_name);
            main_content = format!("{}\n#include<fstream>\n\nusing namespace std;\n\nifstream in(\"{}\");", main_content, input_name);
            io_content = String::from("touch $(INPUT)");

            skin.print_inline(&format!("**[{}/{}]** Successfully created input file: *{}*\n", process_step, NUM_STEPS, input_name));
        },
        None => skin.print_inline(&format!("**[{}/{}]** No input file provided\n", process_step, NUM_STEPS)),
    }

    process_step += 1;

    match opt.output.clone() {
        Some(output_name) => {
            is_output_file = true;

            let mut path = opt.name.clone();
            path.push(output_name.clone());
            match File::create(&path) {
                Err(why) => panic!("Couldn't create {}: {}", path.display(), why),
                Ok(_) => (),
            };

            if is_input_file {
                main_content = format!("{}\nofstream out(\"{}\");", main_content, output_name);
                makefile_content = format!("{}\nOUTPUT = {}", makefile_content, output_name);
                io_content = format!("{} && touch $(OUTPUT)", io_content);
            } else {
                main_content = format!("{}\n#include<fstream>\n\nusing namespace std;\n\nofstream out(\"{}\");", main_content, output_name);
                makefile_content = format!("{}\n\n# I/O files\nOUTPUT = {}", makefile_content, output_name);
                io_content = String::from("touch $(OUTPUT)");
            }

            skin.print_inline(&format!("**[{}/{}]** Successfully created output file: *{}*\n", process_step, NUM_STEPS, output_name));
        },
        None => skin.print_inline(&format!("**[{}/{}]** No output file provided\n", process_step, NUM_STEPS)),
    }

    process_step += 1;

    if !is_input_file && !is_output_file { main_content = format!("{}\n\nusing namespace std;", main_content) }

    makefile_content = format!("{}\n\nall: $(TARGET)", makefile_content);
    makefile_content = format!("{}\n\t$(CC) $(CFLAGS) -o $(TARGET) $(TARGET).cpp && ./$(TARGET)", makefile_content);
    if is_input_file || is_output_file { makefile_content = format!("{}\n\nio:\n\t{}", makefile_content, io_content); }
    makefile_content = format!("{}\n\nclean:\n\t$(RM) $(TARGET)", makefile_content);

    main_content = format!("{}\n\nint main() {{\n\tcout << \"Hello World!\" << endl;\n\treturn 0;\n}}", main_content);

    match create_file(&opt.name, String::from("main.cpp"), main_content, &mut process_step) {
        Ok(_) => (),
        Err(_) => (),
    }

    match create_file(&opt.name, String::from("Makefile"), makefile_content, &mut process_step) {
        Ok(_) => (),
        Err(_) => (),
    }
}

fn create_file(path: &PathBuf, name: String, content: String, process_step: &mut usize) -> Result<(), Box<dyn Error>>{
    let mut skin = MadSkin::default();
    skin.italic.set_fg(rgb(139, 0, 139));
    skin.bold.set_fg(rgb(50, 205, 50));

    let mut path = path.clone();

    path.push(name.clone());

    let mut file = match File::create(&path) {
        Ok(file) => {
            skin.print_inline(&format!("**[{}/{}]** Successfully created: *{}*\n", process_step, NUM_STEPS, name.clone()));
            file
        },
        Err(error) => {
            skin.print_inline(&format!("**[{}/{}]** Failed to create *{}*: {}\n", process_step, NUM_STEPS, name.clone(), error));
            *process_step += 1;
            return Err(Box::from(error))
        },
    };

    *process_step += 1;

    match file.write_all(content.as_bytes()) {
        Ok(_) => {
            skin.print_inline(&format!("**[{}/{}]** Successfully initialized: *{}*\n", process_step, NUM_STEPS, name.clone()));
        },
        Err(error) => {
            skin.print_inline(&format!("**[{}/{}]** Failed to initialize *{}*: {}\n", process_step, NUM_STEPS, name.clone(), error));
            *process_step += 1;
            return Err(Box::from(error))
        },
    }

    *process_step += 1;

    Ok(())
}









