use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("A command must be specified. Run 'next-butler help' to see what you can do");
        process::exit(1);
    }

    match args[1].as_str() {
        "help" => {
            println!("\
                Commands:\n\
                \thelp: Show what you're reading now\n\
                \tnew {{page|style|component}} FILE_NAME: Create a new page, style or component file\n\n\
                For more info read the README\n\
            ");
        },
        "new" => {
            let file_type = args.get(2);
            let file_name = args.get(3);
            if file_type.is_none() || file_name.is_none() {
                eprintln!("\
                    A type of file and a name MUST be provided\n\
                    next-butler new {{page|style|component}} FILE_NAME\
                ");
            }
        }
        _ => {
            eprintln!("Command not found. Run 'next-butler help' to see what you can do");
        }
    }

    next_butler::run(&args);
}
