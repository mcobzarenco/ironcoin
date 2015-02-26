use std::fs::File;
use std::io::{Read, Write};
use std::iter::count;
use std::path::Path;

use protobuf::{self, MessageStatic, RepeatedField};
use rustc_serialize::base64::{self, ToBase64};
use sodiumoxide::crypto::sign::ed25519;

use crypto::{PublicKey, SecretKey};
use error::IroncResult;
use ironcoin_pb::{Wallet, WalletKeypair};

pub fn load_proto_from_file<Message: MessageStatic>(
    path: &str) -> IroncResult<Message>
{
    let mut proto_in = try!(File::open(&Path::new(path)));
    let mut wallet_bytes = vec![];
    try!(proto_in.read_to_end(&mut wallet_bytes));
    Ok(try!(protobuf::parse_from_bytes(&wallet_bytes)))
}

pub fn save_proto_to_file<Message: MessageStatic>(
    path: &str, msg: &Message) -> IroncResult<()>
{
    let mut proto_out = try!(File::create(&Path::new(path)));
    let wallet_bytes = try!(msg.write_to_bytes());
    Ok(try!(proto_out.write_all(&wallet_bytes)))
}

pub fn load_from_file(path: &str) -> IroncResult<Wallet> {
    load_proto_from_file(path)
}

pub fn save_to_file(path: &str, wallet: &Wallet) -> IroncResult<()> {
    save_proto_to_file(path, wallet)
}

pub fn pretty_format(wallet_key: &WalletKeypair) -> String {
    let mut formatted = String::new();
    let pk = wallet_key.get_public_key();
    let sk = wallet_key.get_secret_key();
    formatted.push_str(&format!(
        "[ {} ]\n", pk.to_base64(base64::STANDARD)));
    formatted.push_str(&format!(
        " name: {}\n", wallet_key.get_name()));
    formatted.push_str(&format!(
        "   pk: {}\n", pk.to_base64(base64::STANDARD)));
    formatted.push_str(&format!(
        "   sk: {}\n", sk.to_base64(base64::STANDARD)));
    formatted
}

pub trait WalletExt {
    fn add_keypair(&mut self, name: &str, public_key: &PublicKey,
                   secret_key: &SecretKey);
    fn add_public_key(&mut self, name: &str, public_key: &PublicKey);
    fn drop_keypairs_no_secret(&mut self);
    fn generate_name(&self) -> String;
    fn generate_new_key(&mut self, name: &str) -> WalletKeypair;
    fn search_keys(&self, search_str: &str) -> Vec<&WalletKeypair>;
}

impl WalletExt for Wallet {
    fn add_keypair(&mut self, name: &str, public_key: &PublicKey,
                   secret_key: &SecretKey) {
        let mut key = WalletKeypair::new();
        key.set_public_key(public_key.0.to_vec());
        key.set_secret_key(secret_key.0.to_vec());
        key.set_name(String::from_str(name));
        self.mut_keypairs().push(key);
    }

    fn add_public_key(&mut self, name: &str, public_key: &PublicKey) {
        let mut key = WalletKeypair::new();
        key.set_public_key(public_key.0.to_vec());
        key.set_name(String::from_str(name));
        self.mut_keypairs().push(key);
    }

    fn drop_keypairs_no_secret(&mut self) {
        let mut keypairs: Vec<WalletKeypair> =
            self.mut_keypairs().clone().into_vec();
        self.set_keypairs(RepeatedField::from_vec(keypairs.drain().filter(
            |kp| kp.decode_secret_key().is_ok()).collect()));
    }

    fn generate_new_key(&mut self, name: &str) -> WalletKeypair {
        let (pk, sk) = ed25519::gen_keypair();
        let mut key = WalletKeypair::new();
        key.set_public_key(pk.0.to_vec());
        key.set_secret_key(sk.0.to_vec());
        key.set_name(String::from_str(name));
        let copy = key.clone();
        self.mut_keypairs().push(key);
        copy
    }

    fn generate_name(&self) -> String {
        let already_exists = |name| -> bool {
            let dups: Vec<&WalletKeypair> =
                self.get_keypairs().iter().filter(|k| {
                    k.get_name() == name
                }).collect();
            dups.len() > 0
        };
        let make_name = |i: usize| { format!("addr{}", i) };
        make_name(count(1, 1)
                  .take_while(|i| {
                      already_exists(make_name(*i))
                  }).last().unwrap_or(0) + 1)
    }

    fn search_keys(&self, search_str: &str) -> Vec<&WalletKeypair> {
        self.get_keypairs().iter()
            .filter(|wkey| {
                let name = &wkey.get_name()[..];
                let pk_base64 = wkey.get_public_key()
                    .to_base64(base64::STANDARD);
                name.starts_with(search_str) ||
                    pk_base64.starts_with(search_str)
            }).collect()
    }
}

pub trait WalletKeypairExt {
    fn decode_public_key(&self) -> IroncResult<PublicKey>;
    fn decode_secret_key(&self) -> IroncResult<SecretKey>;
}

impl WalletKeypairExt for WalletKeypair {
    fn decode_public_key(&self) -> IroncResult<PublicKey> {
        PublicKey::from_slice(self.get_public_key())
    }

    fn decode_secret_key(&self) -> IroncResult<SecretKey> {
        SecretKey::from_slice(self.get_secret_key())
    }
}

/*****  Tests  *****/

use crypto::gen_keypair;

#[test]
fn test_wallet_ext_add_key() {
    let (pk1, sk1) = gen_keypair();
    let (pk2, sk2) = gen_keypair();
    let mut wallet = Wallet::new();
    assert_eq!(0, wallet.get_keypairs().len());
    wallet.add_keypair("cheia_lu_mata", &pk1, &sk1);
    assert_eq!(1, wallet.get_keypairs().len());
    wallet.add_keypair("cheia_lu_tactu", &pk2, &sk2);
    assert_eq!(2, wallet.get_keypairs().len());

    assert_eq!(pk1, wallet.get_keypairs()[0].decode_public_key().unwrap());
    assert_eq!(sk1, wallet.get_keypairs()[0].decode_secret_key().unwrap());
}

#[test]
fn test_wallet_ext_add_public_key() {

}
