use fast_command::{Output, Command};

fn main() -> std::io::Result::<()> {
    let Output { status, stdout, stderr } = Command::new("echo hello").execute()?;

    println!("status = {status}");
    println!("stdout = {stdout:?}");
    println!("stderr = {stderr:?}");

    Ok(())
}
