use std::process::Command;
use std::str;
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


pub fn docker_kill(port: i32) {
    let mut command: String = "".to_owned();
    command.push_str(&format!(
        "docker kill dind_port{} && docker container prune -f",
        port.to_string()
    ));
    let mut c = command_wrapper(&command, "/tmp/");
    let c_out = c.output().expect("docker run failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
}

pub fn docker_run_new_test_instance(password: &str, port: i32) -> String {
    let mut command: String = "".to_owned();
    command.push_str(&format!(
        "docker run --privileged \
        -e CODER_PASSWORD=\"{}\" \
        -p {}:8443 \
        --name dind_port{} \
        -d dind_base",
        password,
        port.to_string(),
        port.to_string()
    ));
    let mut c = command_wrapper(&command, "/tmp/");
    let c_out = c.output().expect("docker run failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out.stdout),
        String::from_utf8_lossy(&c_out.stderr)
    );
    let mut command2: String = "".to_owned();
    command2.push_str(&format!(
        "docker exec dind_port{} /bin/ash -c ./start_dind.sh &",
        port.to_string()
    ));
    let mut c2 = command_wrapper(&command2, "/tmp/");
    let c_out2 = c2.output().expect("docker run failed");
    println!(
        "STD_OUT\n{}\nSTDERR\n{}",
        String::from_utf8_lossy(&c_out2.stdout),
        String::from_utf8_lossy(&c_out2.stderr)
    );
    String::from_utf8_lossy(&c_out.stdout).to_string()
}
