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

        println!("Authenticated!");
        
    }


}



//TODO: Func to run a binary on said machine??

//TODO: Func to spin down a machine given a VM name

//helper func for parsing private key
/* 
async fn parse_private_key(file_path: String) -> Result<String, io::Error>{

    //read the private key file
    let contents: Result<String, io::Error> = fs::read_to_string(file_path).unwrap()?;



}
    */