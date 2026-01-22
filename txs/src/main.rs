use bitcoin::{Address, PublicKey, Network, PrivateKey};
use bitcoin::secp256k1::{rand, Secp256k1};

fn main() {
//generate new keypair 
let s = Secp256k1::new();
// let public_key = PublicKey::new(s.generate_keypair(&mut rand::thread_rng()).1);
let (secret_key, public_key) = s.generate_keypair(&mut rand::thread_rng());
let pubk= PublicKey::new(public_key);
let pvtk= PrivateKey::new(secret_key, Network::Bitcoin);
let address = Address::p2pkh(&pubk, Network::Bitcoin);
println!("Address: {address}");
//pvt key print(just for test)
println!("Secret Key: {pvtk}");
}
