use std::sync::Arc;
//use tokio as tk;
use russh::{client, ChannelId};
//use russh_keys::key;
//use anyhow::Error;
use async_trait::async_trait;
use log::info;
use russh_sftp::client::SftpSession;


//things to help with connecting to vm




//helper function to spin_up_machine
//params: username - name of the user on the vm
//        vm_ip - ip address of the vm we are trying to connect to
//        pass = password for the username on the vm
//return: Result holding either a successful connection or the error thrown during the process

 

async fn connect_to_machine(username: String, vm_ip: String, pass: String) {

    struct Client;

    #[async_trait]
    impl client::Handler for Client {
        type Error = anyhow::Error;

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

    //??
    env_logger::init();

    //set up configuration for the session
    let config = russh::client::Config::default();
    let sh = Client {};

    //connect to the session
    let mut session = russh::client::connect(Arc::new(config), (vm_ip, 22), sh).await.unwrap();

    //if session auth works then ...
    if session.authenticate_password(username, pass).await.unwrap() {
        //open channel on session
        let channel = session.channel_open_session().await.unwrap();

        //request sftp subsystem??
        channel.request_subsystem(true, "sftp").await.unwrap();

        //??
        let sftp = SftpSession::new(channel.into_stream()).await.unwrap();
        info!("current path: {:?}", sftp.canonicalize(".").await.unwrap());

        // create dir and symlink
        let path = "./jjs_special_dir";
        let symlink = "./symlink";

        //sftp the directory
        sftp.create_dir(path).await.unwrap();
        sftp.symlink(path, symlink).await.unwrap();

        //??
        info!("dir info: {:?}", sftp.metadata(path).await.unwrap());
        info!(
            "symlink info: {:?}",
            sftp.symlink_metadata(path).await.unwrap()
        );

        // scanning directory
        for entry in sftp.read_dir(".").await.unwrap() {
            info!("file in directory: {:?}", entry.file_name());
        }

        //remove things?
        sftp.remove_file(symlink).await.unwrap();
        sftp.remove_dir(path).await.unwrap();
        
    }


}



//TODO: Func to run a binary on said machine??

//TODO: Func to spin down a machine given a VM name
