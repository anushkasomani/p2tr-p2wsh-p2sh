use bitcoin::{
    absolute, address::Address, Amount, CompressedPublicKey, Network, 
    OutPoint, PrivateKey, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, Witness
};
use bitcoin::secp256k1::{rand, Secp256k1};
use std::str::FromStr;

fn main() {
    let s = Secp256k1::new();
    
    // 1. Generate the raw keypair
    let (secret_key, public_key) = s.generate_keypair(&mut rand::thread_rng());

    // 2. Wrap them into the specific types Bitcoin expects
    // We use CompressedPublicKey because P2WPKH (SegWit) requires it
    let compressed_pubk = CompressedPublicKey(public_key);
    let pvtk = PrivateKey::new(secret_key, Network::Bitcoin);

    // 3. Create the SegWit Address (bc1q...)
    let address = Address::p2wpkh(&compressed_pubk, Network::Bitcoin);

    // 4. Set up transaction placeholders
    // Using a dummy TXID for now so it compiles
    let txid_to_spend = Txid::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    
    // 5. Construct the Transaction
    let input = TxIn {
        previous_output: OutPoint::new(txid_to_spend, 0),
        script_sig: ScriptBuf::new(), // Must be empty for Native SegWit
        sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
        witness: Witness::new(), // This is where the signature goes later
    };

    let output = TxOut {
        value: Amount::from_sat(50_000), // 0.0005 BTC
        script_pubkey: address.script_pubkey(), // Sending back to yourself for this test
    };

    let tx = Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input: vec![input],
        output: vec![output],
    };

    println!("Generated SegWit Address: {}", address);
    println!("Private Key (WIF): {}", pvtk);
    println!("Transaction total outputs: {} sats", tx.output[0].value);
}