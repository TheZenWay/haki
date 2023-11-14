use std::process::Command;

pub fn install() {
    println!("Installing Pinia...");

    let npm_install = Command::new("npm")
        .arg("install")
        .arg("pinia")
        .spawn();

    match npm_install {
        Ok(mut child) => {
            let status = child.wait().expect("Failed to wait for npm install");

            if status.success() {
                let mkdir_status = Command::new("mkdir")
                    .arg("store")
                    .status()
                    .expect("Failed to execute mkdir command");

                if mkdir_status.success() {
                    let mv_status = Command::new("mv")
                        .arg("store.js")
                        .arg("./store/store.js")
                        .status()
                        .expect("Failed to execute mv command");

                    if mv_status.success() {
                        println!("Pinia successfully installed!");
                    } else {
                        println!("Failed to move store.js. mv command failed.");
                    }
                } else {
                    println!("Failed to create the 'store' directory. mkdir command failed.");
                }
            } else {
                println!("Failed to install Pinia. npm install command failed.");
            }
        }
        Err(e) => {
            println!("Failed to run npm install: {}", e);
        }
    }
}
