use std::old_io::{File, Open, Read, Write, ReadWrite};
use std::vec;

use protobuf;
use rustc_serialize::base64::{self, ToBase64};
use sodiumoxide::crypto::sign::ed25519;

use error::{SimplesResult};
use simples_pb::{Wallet, WalletKey};

fn load_proto_from_file<Message: protobuf::MessageStatic>(
    path: &str) -> SimplesResult<Message>
{
    let mut proto_in = File::open(&Path::new(path));
    let wallet_bytes = try!(proto_in.read_to_end());
    Ok(try!(protobuf::parse_from_bytes(&wallet_bytes[])))
}

fn save_proto_to_file<Message: protobuf::MessageStatic>(
    path: &str, msg: &Message) -> SimplesResult<()>
{
    let mut proto_out = File::create(&Path::new(path));
    let wallet_bytes = try!(msg.write_to_bytes());
    Ok(try!(proto_out.write(&wallet_bytes[])))
}

pub fn load_from_file(path: &str) -> SimplesResult<Wallet> {
    load_proto_from_file(path)
}

pub fn save_to_file(path: &str, wallet: &Wallet) -> SimplesResult<()> {
    save_proto_to_file(path, wallet)
}

pub fn pretty_format(wallet_key: &WalletKey) -> String {
    let mut formatted = String::new();
    let pk = wallet_key.get_public_key();
    let sk = wallet_key.get_secret_key();
    formatted.push_str(&format!(
        "[ {} ]\n", pk.to_base64(base64::STANDARD))[]);
    formatted.push_str(&format!(
        " name: {}\n", wallet_key.get_name())[]);
    formatted.push_str(&format!(
        " desc: {}\n", wallet_key.get_description())[]);
    formatted.push_str(&format!(
        "   pk: {}\n", pk.to_base64(base64::STANDARD))[]);
    formatted.push_str(&format!(
        "   sk: {}\n", sk.to_base64(base64::STANDARD))[]);
    formatted
}

pub trait WalletExt {
    fn generate_new_key(
        &mut self, name: &str, description: &str) -> WalletKey;
    fn get_keys_by_name(&self, search_str: &str) -> Vec<&WalletKey>;
}

impl WalletExt for Wallet {
    fn generate_new_key(
        &mut self, name: &str, description: &str) -> WalletKey
    {
        let (pk, sk) = ed25519::gen_keypair();
        let mut key = WalletKey::new();
        key.set_public_key(pk.0.to_vec());
        key.set_secret_key(sk.0.to_vec());
        key.set_name(String::from_str(name));
        key.set_description(String::from_str(description));
        let copy = key.clone();
        self.mut_keys().push(key);
        copy
    }

    fn get_keys_by_name(&self, search_str: &str) -> Vec<&WalletKey> {
        let mut keys = Vec::<&WalletKey>::new();
        self.get_keys().iter()
            .filter(|wkey| {
                let name = &wkey.get_name()[];
                let matches: Vec<(usize, usize)> =
                    name.match_indices(search_str).collect();
                matches.len() > 0
            }).collect()
    }
}
