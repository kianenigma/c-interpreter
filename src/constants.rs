pub static PROMPT: &'static str = ">>> ";
pub static ASCII: &'static str = r#"
  /$$$$$$                      /$$$$$$ /$$   /$$ /$$$$$$$$/$$$$$$$$ /$$$$$$$  /$$$$$$$  /$$$$$$$  /$$$$$$$$/$$$$$$$$/$$$$$$$$ /$$$$$$$ 
 /$$__  $$                    |_  $$_/| $$$ | $$|__  $$__/ $$_____/| $$__  $$| $$__  $$| $$__  $$| $$_____/__  $$__/ $$_____/| $$__  $$
| $$  \__/                      | $$  | $$$$| $$   | $$  | $$      | $$  \ $$| $$  \ $$| $$  \ $$| $$        | $$  | $$      | $$  \ $$
| $$             /$$$$$$        | $$  | $$ $$ $$   | $$  | $$$$$   | $$$$$$$/| $$$$$$$/| $$$$$$$/| $$$$$     | $$  | $$$$$   | $$$$$$$/
| $$            |______/        | $$  | $$  $$$$   | $$  | $$__/   | $$__  $$| $$____/ | $$__  $$| $$__/     | $$  | $$__/   | $$__  $$
| $$    $$                      | $$  | $$\  $$$   | $$  | $$      | $$  \ $$| $$      | $$  \ $$| $$        | $$  | $$      | $$  \ $$
|  $$$$$$/                     /$$$$$$| $$ \  $$   | $$  | $$$$$$$$| $$  | $$| $$      | $$  | $$| $$$$$$$$  | $$  | $$$$$$$$| $$  | $$
 \______/                     |______/|__/  \__/   |__/  |________/|__/  |__/|__/      |__/  |__/|________/  |__/  |________/|__/  |__/
"#;

pub const HELP: &'static str = r#"
Supported statements:
    - Includes
    - Defines 
    - Functions (must be prepended with `#fun`)
    - Normal Statement.
Commands: 
    ~src           - View the soruce code.
    ~run           - Run current source code.
    ~del <X>       - Delete a statement 
    ~arg <X?>      - Get/Set a command line argument (argv)
    ~xcc <X?>      - Get/Set the compiler.

by Kianenigma.
"#;

pub const PROGRAM_TEMPLATE: &'static str = r#"{includes}

{defines}

{functions}

int main(int argc, char **argv) {{
// statements 
{statements}

    printf("Hello C-Interpreter!\n");
    return 0;
}}"#;

pub const TEMP_FILE: &'static str = "temp.c";