use replicator_lib::connect_to_machine as con_to_mach;
use std::{ env, fs, io };
//use tokio::*;

#[tokio::main]
async fn main() {

    //TODO: Get file name from CLA
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];

    //handle error of invalid file
    let contents_r: Result<String, io::Error> = fs::read_to_string(file_path);
    let contents: String;

    match contents_r {
        Ok(val) => contents = val,
        Err(error) => panic!("Threw the following error: {}", error)
    }

    //TODO: Also consider posibility of trouble reading file


    //Creates a vector where each tuple in the vec holds a vm_ip
    //and path to the ssh key
    let vms: Vec<(String, String)> = contents.lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            if let (Some(vm_ip), Some(ssh_key)) =(parts.next(), parts.next()) {
                Some((vm_ip.to_string(), ssh_key.to_string()))
            } else {
                None
            }

        }).collect();


    //TODO: Call lib func to connect to machine from vec holding VM names
    // params: vm_name, path_to_key
    env_logger::init();

    for machine in &vms {
        con_to_mach(machine.0.to_string(), machine.1.to_string()).await;
    }
    //con_to_mach(vms[0].0.to_string(), vms[0].1.to_string()).await;
    //con_to_mach(vms[1].0.to_string(), vms[1].1.to_string()).await;

    //TODO: Check if binary is already installed
    //          if not, install binary

    //TODO: Run the binary

    //TODO: Call lib func to spin down machine from vec holding VM names
}
