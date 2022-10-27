use std::process::Command;

fn main() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
        .args(["-a", "echo hellocmd"])
        .output()
        .expect("failed to execute process")
    } else {
        Command::new("sh")
        .arg("-c")
        .arg("echo hellosh!")
        .output()
        .expect("failed to execute process")
    };
    let hello = output.stdout;
    println!("{:?}", hello);
}
