use secp256k1::{Secp256k1, All, key, Message, Signature};
use secp256k1::rand::rngs;

pub struct KeyPair {
    pub public_key: key::PublicKey,
    private_key: key::SecretKey,
    secp: Secp256k1<All>,
}


impl KeyPair {
    pub fn sign(&self, messages_bytes: &[u8]) -> Signature {
        let message = match Message::from_slice(messages_bytes) {
            Ok(msg) => msg,
            Err(e) => panic!("{}", e)
        };

        self.secp.sign(&message, &self.private_key)
    }
}

pub fn create_key_pair() -> KeyPair {
    let mut rand = rngs::OsRng::new().unwrap();
    let secp = Secp256k1::new();

    let (private_key, public_key) = secp.generate_keypair(&mut rand);
    return KeyPair {
        secp,
        private_key,
        public_key,
    };
}
