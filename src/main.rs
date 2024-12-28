use std::io::{self, Write};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let home = env::var("HOME")
        .unwrap_or_else(|err| panic!("ERROR: Unable to retrieve home variable\n{err}"));

    let mut problem_name = String::new();

    if args.len() < 2 {
        print!("Input the name of the problem: ");
        // Flush the output so the user sees the prompt immediately
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut problem_name)
            .expect("Failed to read problem name");

        problem_name = problem_name.trim().replace(' ', "_");
    } else if args.len() == 2 {
        problem_name = args[1].replace(' ', "_");
    } else {
        println!("ERROR: Incorrect usage");
        println!("=> slv <problem_name>");
        return;
    }

    let exe_path = ".exe/";
    let template_path = format!("{home}/fillthehole/templates/templates.cpp");

    let make_content = format!(
        "{problem_name}: {problem_name}.cpp\n\tg++ {problem_name}.cpp -o {exe_path}{problem_name} && {exe_path}{problem_name}\n\nclean:\n\trm -I {exe_path}*"
    );

    let program_path = format!("{problem_name}.cpp");
    let template_content = fs::read_to_string(template_path)
        .unwrap_or_else(|err| panic!("ERROR: unable to read template\n{err}"));
    fs::write(&program_path, template_content)
        .unwrap_or_else(|err| panic!("ERROR: unable to copy from template\n{err}"));

    println!("Made {program_path} using templates.cpp");

    fs::create_dir_all(exe_path)
        .unwrap_or_else(|err| panic!("ERROR: unable to create exe path\n{err}"));
    fs::write("./makefile", make_content)
        .unwrap_or_else(|err| panic!("ERROR: unable to write to makefile\n{err}"));

    println!("Updated makefile to run {program_path}");
}
