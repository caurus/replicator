use std::sync::Arc;
//use std::io;
//use std::io::Write;
//use tokio::io as tokio_io;
use russh::*;
use russh::keys::*;
use russh_keys::decode_secret_key;
//use russh_keys::key::KeyPair;
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
 


pub async fn connect_to_machine(vm_ip: String, path_to_key: String) {

    //??
    //env_logger::init();
   

    //set up configuration for the session
    let config = russh::client::Config::default();
    let sh = Client {};

    let private_key = tokio::fs::read_to_string(path_to_key).await.unwrap();
    let keypair = decode_secret_key(&private_key, None).unwrap();
    //connect to the session
    let mut session = russh::client::connect(Arc::new(config), (vm_ip, 22), sh).await.unwrap();

    //if session auth works then ...
    //TODO: Must change this to use russh key 
    if session.authenticate_publickey("ubuntu", Arc::new(keypair)).await.unwrap() {
        //open channel on session
        let channel = session.channel_open_session().await.unwrap();

        //begin sftp session
        channel.request_subsystem(true, "sftp").await.unwrap();

        //instantiate sftp session
        let sftp = SftpSession::new(channel.into_stream()).await.unwrap();

        //print path of sftp on vm
        info!("current path: {:?}", sftp.canonicalize(".").await.unwrap());

        let path_on_vm = "./new_dir";

        //create a new directory on remote machine
        //sftp.create_dir(path_on_vm).await.unwrap();

        //deelte a directory on remote machine
        //sftp.remove_dir(path_on_vm).await.unwrap();


        //copy something over to the machine


        
    }


}



//TODO: Func to run a binary on said machine??

//TODO: Func to spin down a machine given a VM name
