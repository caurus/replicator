use async_trait::async_trait;
use log::info;
use russh::keys::*;
use russh::*;
use russh_keys::decode_secret_key;
use russh_sftp::client::SftpSession;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

// things to help with connecting to vm

// helper function to spin_up_machine
// params: username - name of the user on the vm
//        vm_ip - ip address of the vm we are trying to connect to
//        pass = password for the username on the vm
// return: Result holding either a successful connection or the error thrown during the process

struct Client;

#[async_trait]
impl client::Handler for Client {
    type Error = anyhow::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &key::PublicKey,
    ) -> Result<bool, Self::Error> {
        info!("check_server_key: {:?}", server_public_key);
        Ok(true)
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        _session: &mut client::Session,
    ) -> Result<(), Self::Error> {
        info!("data on channel {:?}: {}", channel, data.len());
        Ok(())
    }
}

pub async fn sftp_to_machine(
    vm_ip: String,
    path_to_key: String,
    test_dir_name: &str,
    binary: Vec<u8>,
) {
    // set up configuration for the session
    let config = russh::client::Config::default();
    let sh = Client {};

    let private_key = tokio::fs::read_to_string(path_to_key).await.unwrap();
    let keypair = decode_secret_key(&private_key, None).unwrap();
    // connect to the session
    let mut session = russh::client::connect(Arc::new(config), (vm_ip, 22), sh)
        .await
        .unwrap();

    // if session auth works then ...
    if session
        .authenticate_publickey("ubuntu", Arc::new(keypair))
        .await
        .unwrap()
    {
        // open channel on session
        let channel = session.channel_open_session().await.unwrap();

        // begin sftp session
        channel.request_subsystem(true, "sftp").await.unwrap();

        // instantiate sftp session
        let sftp = SftpSession::new(channel.into_stream()).await.unwrap();

        // print path of sftp on vm
        info!("current path: {:?}", sftp.canonicalize(".").await.unwrap());

        // format the test_dir name
        let path_to_dir = format!("./{}", &test_dir_name);

        // check if the directory already exists on the vm
        match sftp.try_exists(&path_to_dir).await {
            Ok(exists) => {
                if exists {
                    // if it exists, check the metadata
                    match sftp.metadata(&path_to_dir).await {
                        Ok(metadata) => {
                            // if metadata is a directory, then print so
                            if metadata.is_dir() {
                                println!("Directory '{}' already exists!", &path_to_dir);
                            } else {
                                println!(
                                    "A file exists at '{}', but it is not a directory.",
                                    &path_to_dir
                                );
                            }
                        }
                        Err(err) => {
                            println!("Error retrieving metadata: {}", err);
                        }
                    }
                } else {
                    // Directory doesn't exist, create it
                    sftp.create_dir(&path_to_dir).await.unwrap();
                    println!("Directory '{}' created.", &path_to_dir);
                }
            }
            Err(err) => {
                println!("Error checking existence: {}", err);
            }
        }

        // at this point, there is a new blank directory on the remote machine

        // path to the new binary
        let path_to_bin = format!("{}/{}", test_dir_name, test_dir_name);

        // match statement to ensure we don't write over another file.
        match sftp.try_exists(&path_to_bin).await {
            Ok(exists) => {
                if exists {
                    // if it already exists, print that it already exists
                    println!("There already exists a file at {}!", &path_to_bin);
                } else {
                    // if it doesn't exist, create the file and copy the binary over
                    // create the file
                    let mut file_on_vm = sftp.create(&path_to_bin).await.unwrap();
                    // write the binary to the file
                    file_on_vm.write_all(&binary).await.unwrap();
                    // ensure the file was written
                    file_on_vm.flush().await.unwrap();

                    println!("File was written at {}!", &path_to_bin);
                }
            }
            Err(err) => {
                print!(
                    "Error checking for the existence of the file. Error: {}",
                    err
                );
            }
        }

        //----------- Code to clean the directory, doesn't work when passing the sftp to another func ---------
        // let path_to_bin = format!("{}/{}.bin", test_dir_name, test_dir_name);
        // sftp.remove_file(path_to_bin).await.unwrap();
        // sftp.remove_dir(path_to_dir).await.unwrap();
        //----------------------------------------------------------------------------------------------------

        //close to sftp connection
        sftp.close().await.unwrap();

        // // must create a new channel, old one is consumed by sftp
        // let new_channel = session.channel_open_session().await.unwrap();
        // // execute the binary
        // let command = "mkdir cmd_from_rust";
        // // is this right?
        // new_channel.exec(false, command).await.unwrap();
    }
}

//TODO: Func to run a binary on said machine??

//TODO: Func to spin down a machine given a VM name
/*
async fn copy_bin_over(sftp: SftpSession, new_dir_name: String, binary: Vec<u8>){

    let path_to_bin = format!("{}/{}.bin", new_dir_name, new_dir_name);

    let mut file_on_vm = sftp.create(&path_to_bin).await.unwrap();

    file_on_vm.write_all(&binary).await.unwrap();

    file_on_vm.flush().await.unwrap();
}
*/

/*
async fn clean(sftp: SftpSession, path_to_dir: String, test_dir_name: String){
    // create path to bin
    let path_to_bin = format!("{}/{}.bin", test_dir_name, test_dir_name);

    // removes file from machine
    sftp.remove_file(path_to_bin).await.unwrap();

    // removes dir from machine.
    sftp.remove_dir(path_to_dir).await.unwrap();
}
*/

