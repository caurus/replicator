use async_trait::async_trait;
use log::info;
use russh::keys::key;
use russh::{
    client,
    ChannelId
};
use russh_keys::decode_secret_key;
use russh_sftp::client::SftpSession;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use anyhow::{
    bail,
    Error
};


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
    binary: &[u8],
) -> anyhow::Result<(), Error> {
    // set up configuration for the session
    let config = russh::client::Config::default();
    let sh = Client {};

    let private_key = tokio::fs::read_to_string(path_to_key).await?;
    let keypair = decode_secret_key(&private_key, None)?;
    // connect to the session
    let mut session = russh::client::connect(Arc::new(config), (vm_ip, 22), sh)
        .await?;


    // if session doesn't authenticate, then bail
    if !session
        .authenticate_publickey("ubuntu", Arc::new(keypair))
        .await?
    {
        bail!("Error authenticating the session!")
    }


    // open channel on session
    let channel = session.channel_open_session().await?;

    // begin sftp session
    channel.request_subsystem(true, "sftp").await?;

    // instantiate sftp session
    let sftp = SftpSession::new(channel.into_stream()).await?;

    // print path of sftp on vm
    info!("current path: {:?}", sftp.canonicalize(".").await?);

    // format the test_dir name
    let path_to_dir = format!("./{}", &test_dir_name);
    // check if the directory already exists on the vm
    copy_dir_to_vm(&sftp, &path_to_dir).await?;


    let path_to_bin = format!("{}/{}", path_to_dir, test_dir_name);
    copy_bin_to_vm(&sftp, &path_to_bin, binary).await?;


    //code to clean the vm
    clean_vm(&sftp, &path_to_bin, &path_to_dir).await?;
    
    //close to sftp connection
    sftp.close().await?;



    // // create a new channel to execute the binary
    // let new_channel = session.channel_open_session().await?;
    // // calling the path to the binary to execute it
    // new_channel.exec(false, path_to_bin).await?;



    Ok(())
    
}

async fn copy_dir_to_vm(sftp: &SftpSession, path_to_dir: &str) -> anyhow::Result<()> {

    match sftp.try_exists(path_to_dir).await {
        Ok(exists) => {
            if exists {
                // if it exists, check the metadata
                match sftp.metadata(path_to_dir).await {
                    Ok(metadata) => {
                        // if metadata is a directory, then print so
                        if metadata.is_dir() {
                            println!("Directory '{}' already exists!", path_to_dir);
                        } else {
                            bail!(
                                "A file exists at '{}', but it is not a directory.",
                                path_to_dir
                            );
                        }
                    }
                    Err(err) => {
                        println!("Error retrieving metadata: {}", err);
                    }
                }
            } else {
                // Directory doesn't exist, create it
                sftp.create_dir(path_to_dir).await?;
                println!("Directory '{}' created.", path_to_dir);
            }
        }
        Err(err) => {
            println!("Error checking existence: {}", err);
        }
    }

    Ok(())
}

async fn copy_bin_to_vm(sftp: &SftpSession, path_to_bin: &str, binary: &[u8]) -> anyhow::Result<()> {

    match sftp.try_exists(path_to_bin).await {
        Ok(exists) => {
            if exists {
                // if it already exists, print that it already exists
                println!("There already exists a file at {}!", path_to_bin);
            } else {
                // if it doesn't exist, create the file and copy the binary over
                // create the file
                let mut file_on_vm = sftp.create(path_to_bin).await?;
                // write the binary to the file
                file_on_vm.write_all(&binary).await?;
                // ensure the file was written
                file_on_vm.flush().await?;

                println!("File was written at {}!", path_to_bin);
            }
        }
        Err(err) => {
            print!(
                "Error checking for the existence of the file. Error: {}",
                err
            );
        }
    }

    Ok(())
}

async fn clean_vm(sftp: &SftpSession, path_to_bin: &str, path_to_dir: &str) -> anyhow::Result<()> {

    sftp.remove_file(path_to_bin).await?;

    sftp.remove_dir(path_to_dir).await?;

    Ok(())
}