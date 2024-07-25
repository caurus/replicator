//use replicator_lib as rl;
use std::{ env, fs, io };

fn main() {
    //rl::phw();

    //TODO: Get file name from CLA
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    //handle error of invalid file
    let contents_r: Result<String, io::Error> = fs::read_to_string(file_path);
    let contents: String;

    match contents_r {
        Ok(val) => contents = val,
        Err(_) => panic!("Invalid file!")
    }

    //TODO: Also consider posibility of trouble reading file


    //TODO: splice the long string by \n to get all the VM names in a vec
    let vm_names: Vec<String> = contents.lines().map(|x| x.to_string()).collect();

    //TODO: Call lib func to spin up machine from vec holding VM names

    //TODO: Check if binary is already installed
    //          if not, install binary

    //TODO: Run the binary

    //TODO: Call lib func to spin down machine from vec holding VM names
}
