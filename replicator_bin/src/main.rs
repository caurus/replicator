use replicator_lib::sftp_to_machine;
use std::{
    env,
    fs::File,
    io::Read,
};

#[tokio::main]
async fn main() -> anyhow::Result<()>{

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <file_path> <test_dir_name_on_vm>", args[0]);
        return Ok(());
    }

    let local_binary_path = 
    "target/x86_64-unknown-linux-musl/release/replicator_bin";
    let file_path = &args[1];
    let test_dir_name = &args[2];


    let vm_info_contents: String = tokio::fs::read_to_string(file_path).await?;

    // Association between machine and private key file to authenticate
    let vm_ips_and_ssh_keys: Vec<(String, String)> = vm_info_contents
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            if let (Some(vm_ip), Some(ssh_key)) = (parts.next(), parts.next()) {
                Some((vm_ip.to_string(), ssh_key.to_string()))
            } else {
                None
            }
        })
        .collect();

    env_logger::init();

    // read binary to be copied
    let mut binary_to_copy = File::open(local_binary_path)?;
    let mut buffer: Vec<u8> = Vec::new();
    binary_to_copy.read_to_end(&mut buffer)?;

    for machine in &vm_ips_and_ssh_keys {
        sftp_to_machine(
            machine.0.to_string(),
            machine.1.to_string(),
            test_dir_name,
            &buffer,
        )
        .await?;
    }

    Ok(())

}