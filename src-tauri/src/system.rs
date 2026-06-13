use std::process::Command;

#[tauri::command]
pub fn check_system<'a>() -> &'a bool {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    if output.status.success() {
        return &true;
    }

    return &false;
}

pub fn check_legendary<'a>() -> &'a bool {
    let output = Command::new("legendary")
        .arg("--help")
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        return &true;
    }

    return &false;
}

pub fn run_checks<'a>() -> &'a bool {
    let system_is_functional = check_system();
    let legendary_is_functional = check_legendary();

    if *system_is_functional {
        println!("System is functional.");
        if *legendary_is_functional {
            println!("Legendary is functional.");
            return &true;
        }
        return &false;
    }
    return &false;
}
