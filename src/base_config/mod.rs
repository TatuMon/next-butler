use std::process;

pub struct BaseConfig {
    pub issued_command: String,
    pub params: Vec<String>,
    // options: Vec<String> Not in use yet
}

impl BaseConfig {
    pub fn build(args: Vec<String>) -> BaseConfig {
        if args.len() < 2 {
            eprintln!("Wrong amount of params. Use 'next-butler help' to see what you can do");
            process::exit(1);
        }

        let issued_command = args[1].clone();
        let mut params: Vec<String> = vec![];
        let mut options: Vec<String> = vec![];
        for arg in &args[2..] {
            if arg.contains("--") {
                options.push(arg.clone());
            } else {
                params.push(arg.clone());
            }
        }

        BaseConfig {
            issued_command,
            params,
            // options
        }
    }
}