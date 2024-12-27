use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let home = env::var("HOME")
        .unwrap_or_else(|err| panic!("ERROR: Unable to retrieve home variable\n{err}"));

    if args.len() < 2 {
        println!("Incorrect usage of command!");
        println!("-> euler <problem_name>");
        return;
    }

    let exe_path = "./exe/";
    let template_path = format!("{home}/fillthehole/templates/templates.cpp");
    let problem_name = &args[1];

    let make_content = format!(
        "{problem_name}: {problem_name}.cpp\n\tg++ {problem_name}.cpp -o {exe_path}{problem_name} && {exe_path}{problem_name}\n\nclean:\n\trm -I {exe_path}*"
    );

    fs::create_dir_all(exe_path)
        .unwrap_or_else(|err| panic!("ERROR: unable to create exe path\n{err}"));
    fs::write("./makefile", make_content)
        .unwrap_or_else(|err| panic!("ERROR: unable to write to makefile\n{err}"));

    let program_path = format!("{problem_name}.cpp");
    let template_content = fs::read_to_string(template_path)
        .unwrap_or_else(|err| panic!("ERROR: unable to read template\n{err}"));

    fs::write(program_path, template_content)
        .unwrap_or_else(|err| panic!("ERROR: unable to copy from template\n{err}"));
}
