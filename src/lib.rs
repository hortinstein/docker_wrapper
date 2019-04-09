use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

//use hubcaps::repositories::{RepoOptions};

//returns a command setup ready to run the tests
pub fn command_wrapper(test_command: &str, command_directory: &str) -> Command {
    let mut command = if cfg!(target_os = "windows") {
        {
            let mut c = Command::new("cmd");
            c.args(&["/C", test_command]);
            c
        }
    } else {
        {
            let mut c = Command::new("sh");
            c.arg("-c");
            c.arg(test_command);
            c
        }
    };
    command.current_dir(command_directory);
    command
}

//rsa key generation
//ssh-keygen -f /etc/ssh/ssh_host_rsa_key -N '' -t rsa
pub fn docker_prune(path: &str) {
    let command = "sudo docker system prune";
    let mut c = command_wrapper(&command, "/tmp/");
    let c_out = c.output().expect("prune failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;
    #[test]
    fn test_create_repo() {

    }

}
