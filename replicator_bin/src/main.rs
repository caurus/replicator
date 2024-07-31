use replicator_lib::connect_to_machine as con_to_mach;
use std::{ env, fs, io };
use std::io::Write;
use rpassword::read_password;
use tokio::*;

#[tokio::main]
async fn main() {

    //TODO: Get file name from CLA
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <file_path> <username>", args[0]);
        return;
    }

    let file_path = &args[1];
    let username = &args[2];

    //handle error of invalid file
    let contents_r: Result<String, io::Error> = fs::read_to_string(file_path);
    let contents: String;

    match contents_r {
        Ok(val) => contents = val,
        Err(error) => panic!("Threw the following error: {}", error)
    }

    //get password from user for VMs
    print!("Please enter your password for the VMs: ");
    io::stdout().flush().unwrap();

    let pass = read_password().unwrap();

    //TODO: Also consider posibility of trouble reading file


    //TODO: splice the long string by \n to get all the VM names in a vec
    let vm_names: Vec<String> = contents.lines().map(|x| x.to_string()).collect();

    //TODO: Call lib func to connect to machine from vec holding VM names
    // params: username, vm_name, pass
    con_to_mach(username.to_string(), vm_names[0].to_string(), pass.to_string()).await;

    //TODO: Check if binary is already installed
    //          if not, install binary

    //TODO: Run the binary

    //TODO: Call lib func to spin down machine from vec holding VM names
}
