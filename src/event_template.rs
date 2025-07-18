use crate::{Event, Kind, PubKey, SecretKey, Signature, Tags, Timestamp, ID};
use secp256k1::{Keypair, SECP256K1};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

/// represents an unsigned nostr event
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventTemplate {
    pub created_at: Timestamp,
    pub kind: Kind,
    pub tags: Tags,
    pub content: String,
}

impl EventTemplate {
    /// returns a signed event with id, pubkey and sig
    pub fn finalize(self, secret_key: &SecretKey) -> Event {
        let pubkey = secret_key.pubkey();

        // create keypair from secret key
        let keypair = Keypair::from_seckey_byte_array(SECP256K1, secret_key.0)
            .expect("should always work because SecretKey should always be valid");

        // serialize and hash the event
        let serialized = self.serialize(pubkey);
        let hash = Sha256::digest(&serialized);

        // sign the hash
        let signature = SECP256K1.sign_schnorr_no_aux_rand(&hash, &keypair);

        Event {
            id: ID::from_bytes(hash.into()),
            pubkey,
            sig: Signature::from_bytes(signature.to_byte_array()),
            kind: self.kind,
            tags: self.tags,
            created_at: self.created_at,
            content: self.content,
        }
    }

    /// serialize the event for ID computation
    pub fn serialize(&self, pubkey: PubKey) -> Vec<u8> {
        let array = serde_json::json!([
            0,
            pubkey.to_hex(),
            self.created_at.0,
            self.kind,
            self.tags,
            self.content
        ]);
        array.to_string().into_bytes()
    }
}

impl fmt::Display for EventTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EventTemplate({}, {}, {}, {})",
            self.kind, self.created_at, self.tags, self.content
        )
    }
}
