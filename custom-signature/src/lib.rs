#![cfg_attr(not(feature = "std"), no_std)]


use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};
use sp_core::{
    crypto::{self, AccountId32, Public},
    ecdsa, ed25519,
    hash::{H256, H512},
    sr25519, RuntimeDebug,
};
use sp_runtime::traits::{IdentifyAccount, Lazy, Verify};
use sp_std::convert::TryFrom;
use sp_std::prelude::*;

// use sp_io::hashing::blake2_256;

/// Signature verify that can work with any known signature types..
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Clone, Encode, Decode, RuntimeDebug)]
pub enum MultiSignature {
    /// An Ed25519 signature.
    Ed25519(ed25519::Signature),
    /// An Sr25519 signature.
    Sr25519(sr25519::Signature),
    /// An ECDSA/SECP256k1 signature.
    Ecdsa(ecdsa::Signature),
}

impl From<ed25519::Signature> for MultiSignature {
    fn from(x: ed25519::Signature) -> Self {
        MultiSignature::Ed25519(x)
    }
}

impl TryFrom<MultiSignature> for ed25519::Signature {
    type Error = ();
    fn try_from(m: MultiSignature) -> Result<Self, Self::Error> {
        if let MultiSignature::Ed25519(x) = m {
            Ok(x)
        } else {
            Err(())
        }
    }
}

impl From<sr25519::Signature> for MultiSignature {
    fn from(x: sr25519::Signature) -> Self {
        MultiSignature::Sr25519(x)
    }
}

impl TryFrom<MultiSignature> for sr25519::Signature {
    type Error = ();
    fn try_from(m: MultiSignature) -> Result<Self, Self::Error> {
        if let MultiSignature::Sr25519(x) = m {
            Ok(x)
        } else {
            Err(())
        }
    }
}

impl From<ecdsa::Signature> for MultiSignature {
    fn from(x: ecdsa::Signature) -> Self {
        MultiSignature::Ecdsa(x)
    }
}

impl TryFrom<MultiSignature> for ecdsa::Signature {
    type Error = ();
    fn try_from(m: MultiSignature) -> Result<Self, Self::Error> {
        if let MultiSignature::Ecdsa(x) = m {
            Ok(x)
        } else {
            Err(())
        }
    }
}

impl Default for MultiSignature {
    fn default() -> Self {
        MultiSignature::Ed25519(Default::default())
    }
}

/// Public key for any known crypto algorithm.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum MultiSigner {
    /// An Ed25519 identity.
    Ed25519(ed25519::Public),
    /// An Sr25519 identity.
    Sr25519(sr25519::Public),
    /// An SECP256k1/ECDSA identity (actually, the Blake2 hash of the compressed pub key).
    Ecdsa(ecdsa::Public),
}

impl Default for MultiSigner {
    fn default() -> Self {
        MultiSigner::Ed25519(Default::default())
    }
}

/// NOTE: This implementations is required by `SimpleAddressDeterminer`,
/// we convert the hash into some AccountId, it's fine to use any scheme.
impl<T: Into<H256>> crypto::UncheckedFrom<T> for MultiSigner {
    fn unchecked_from(x: T) -> Self {
        ed25519::Public::unchecked_from(x.into()).into()
    }
}

impl AsRef<[u8]> for MultiSigner {
    fn as_ref(&self) -> &[u8] {
        match *self {
            MultiSigner::Ed25519(ref who) => who.as_ref(),
            MultiSigner::Sr25519(ref who) => who.as_ref(),
            MultiSigner::Ecdsa(ref who) => who.as_ref(),
        }
    }
}

impl IdentifyAccount for MultiSigner {
    type AccountId = AccountId32;
    fn into_account(self) -> AccountId32 {
        match self {
            MultiSigner::Ed25519(who) => <[u8; 32]>::from(who).into(),
            MultiSigner::Sr25519(who) => <[u8; 32]>::from(who).into(),
            MultiSigner::Ecdsa(who) => sp_io::hashing::blake2_256(&who.as_ref()[..]).into(),
        }
    }
}

impl From<ed25519::Public> for MultiSigner {
    fn from(x: ed25519::Public) -> Self {
        MultiSigner::Ed25519(x)
    }
}

impl TryFrom<MultiSigner> for ed25519::Public {
    type Error = ();
    fn try_from(m: MultiSigner) -> Result<Self, Self::Error> {
        if let MultiSigner::Ed25519(x) = m {
            Ok(x)
        } else {
            Err(())
        }
    }
}

impl From<sr25519::Public> for MultiSigner {
    fn from(x: sr25519::Public) -> Self {
        MultiSigner::Sr25519(x)
    }
}

impl TryFrom<MultiSigner> for sr25519::Public {
    type Error = ();
    fn try_from(m: MultiSigner) -> Result<Self, Self::Error> {
        if let MultiSigner::Sr25519(x) = m {
            Ok(x)
        } else {
            Err(())
        }
    }
}

impl From<ecdsa::Public> for MultiSigner {
    fn from(x: ecdsa::Public) -> Self {
        MultiSigner::Ecdsa(x)
    }
}

impl TryFrom<MultiSigner> for ecdsa::Public {
    type Error = ();
    fn try_from(m: MultiSigner) -> Result<Self, Self::Error> {
        if let MultiSigner::Ecdsa(x) = m {
            Ok(x)
        } else {
            Err(())
        }
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for MultiSigner {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MultiSigner::Ed25519(ref who) => write!(fmt, "ed25519: {}", who),
            MultiSigner::Sr25519(ref who) => write!(fmt, "sr25519: {}", who),
            MultiSigner::Ecdsa(ref who) => write!(fmt, "ecdsa: {}", who),
        }
    }
}

impl Verify for MultiSignature {
    type Signer = MultiSigner;
    fn verify<L: Lazy<[u8]>>(&self, mut msg: L, signer: &AccountId32) -> bool {
        match (self, signer) {
            (MultiSignature::Ed25519(ref sig), who) => {
                sig.verify(msg, &ed25519::Public::from_slice(who.as_ref()))
            }
            (MultiSignature::Sr25519(ref sig), who) => {
                sig.verify(msg, &sr25519::Public::from_slice(who.as_ref()))
            }
            (MultiSignature::Ecdsa(ref sig), who) => {
                let m = sp_io::hashing::blake2_256(msg.get());
                match sp_io::crypto::secp256k1_ecdsa_recover_compressed(sig.as_ref(), &m) {
                    Ok(pubkey) => {
                        &sp_io::hashing::blake2_256(pubkey.as_ref()) == <dyn AsRef<[u8; 32]>>::as_ref(who)
                    }
                    _ => false,
                }
            }
        }
    }
}

/// Signature verify that can work with any known signature types..
#[derive(Eq, PartialEq, Clone, Default, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AnySignature(H512);

impl Verify for AnySignature {
    type Signer = sr25519::Public;
    fn verify<L: Lazy<[u8]>>(&self, mut msg: L, signer: &sr25519::Public) -> bool {
        let msg = msg.get();
        sr25519::Signature::try_from(self.0.as_fixed_bytes().as_ref())
            .map(|s| s.verify(msg, signer))
            .unwrap_or(false)
            || ed25519::Signature::try_from(self.0.as_fixed_bytes().as_ref())
                .map(|s| s.verify(msg, &ed25519::Public::from_slice(signer.as_ref())))
                .unwrap_or(false)
    }
}

impl From<sr25519::Signature> for AnySignature {
    fn from(s: sr25519::Signature) -> Self {
        AnySignature(s.into())
    }
}

impl From<ed25519::Signature> for AnySignature {
    fn from(s: ed25519::Signature) -> Self {
        AnySignature(s.into())
    }
}
