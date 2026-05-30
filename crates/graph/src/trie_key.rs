use uuid::Uuid;

use crate::digest::Digest;
use crate::model::EdgeType;

pub const CRDT_SUFFIX_LEN: usize = 72;
pub const BLOCK_ENTITY_PREFIX_LEN: usize = 16;
pub const BLOCK_KEY_LEN: usize = 88;
pub const EDGE_ENTITY_PREFIX_LEN: usize = 33;
pub const EDGE_NAV_PREFIX_LEN: usize = 17;
pub const EDGE_KEY_LEN: usize = 105;

const ZERO_DIGEST: Digest = [0u8; 32];

pub struct CrdtKeySuffix;

impl CrdtKeySuffix {
    pub fn write_into(
        out: &mut [u8; CRDT_SUFFIX_LEN],
        version: u64,
        digest: &Digest,
        previous_digest: Option<&Digest>,
    ) {
        out[..8].copy_from_slice(&version.to_be_bytes());
        out[8..40].copy_from_slice(digest);
        out[40..72].copy_from_slice(previous_digest.unwrap_or(&ZERO_DIGEST));
    }

    pub fn read_version(bytes: &[u8]) -> u64 {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&bytes[..8]);
        u64::from_be_bytes(buf)
    }

    pub fn read_digest(bytes: &[u8]) -> Digest {
        let mut digest = [0u8; 32];
        digest.copy_from_slice(&bytes[8..40]);
        digest
    }

    pub fn read_previous_digest(bytes: &[u8]) -> Option<Digest> {
        let mut digest = [0u8; 32];
        digest.copy_from_slice(&bytes[40..72]);
        if digest == ZERO_DIGEST {
            None
        } else {
            Some(digest)
        }
    }
}

pub fn block_entity_prefix(id: Uuid) -> [u8; BLOCK_ENTITY_PREFIX_LEN] {
    *id.as_bytes()
}

pub fn block_version_key(
    id: Uuid,
    version: u64,
    digest: &Digest,
    previous_digest: Option<&Digest>,
) -> [u8; BLOCK_KEY_LEN] {
    let mut key = [0u8; BLOCK_KEY_LEN];
    key[..BLOCK_ENTITY_PREFIX_LEN].copy_from_slice(id.as_bytes());
    CrdtKeySuffix::write_into(
        (&mut key[BLOCK_ENTITY_PREFIX_LEN..]).try_into().expect("suffix"),
        version,
        digest,
        previous_digest,
    );
    key
}

pub fn edge_entity_prefix(target: Uuid, edge_type: EdgeType, source: Uuid) -> [u8; EDGE_ENTITY_PREFIX_LEN] {
    let mut prefix = [0u8; EDGE_ENTITY_PREFIX_LEN];
    prefix[..16].copy_from_slice(target.as_bytes());
    prefix[16] = edge_type as u8;
    prefix[17..33].copy_from_slice(source.as_bytes());
    prefix
}

pub fn edge_nav_prefix(target: Uuid, edge_type: EdgeType) -> [u8; EDGE_NAV_PREFIX_LEN] {
    let mut prefix = [0u8; EDGE_NAV_PREFIX_LEN];
    prefix[..16].copy_from_slice(target.as_bytes());
    prefix[16] = edge_type as u8;
    prefix
}

pub fn edge_version_key(
    target: Uuid,
    edge_type: EdgeType,
    source: Uuid,
    version: u64,
    digest: &Digest,
    previous_digest: Option<&Digest>,
) -> [u8; EDGE_KEY_LEN] {
    let mut key = [0u8; EDGE_KEY_LEN];
    key[..EDGE_ENTITY_PREFIX_LEN]
        .copy_from_slice(&edge_entity_prefix(target, edge_type, source));
    CrdtKeySuffix::write_into(
        (&mut key[EDGE_ENTITY_PREFIX_LEN..]).try_into().expect("suffix"),
        version,
        digest,
        previous_digest,
    );
    key
}

pub fn block_version_key_from(version: &crate::version::BlockVersion) -> [u8; BLOCK_KEY_LEN] {
    block_version_key(
        version.id,
        version.version,
        &version.digest,
        version.previous_digest.as_ref(),
    )
}

pub fn edge_version_key_from(version: &crate::version::EdgeVersion) -> [u8; EDGE_KEY_LEN] {
    edge_version_key(
        version.target,
        version.edge_type,
        version.source,
        version.version,
        &version.digest,
        version.previous_digest.as_ref(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_lengths_and_version_ordering() {
        let id = Uuid::new_v4();
        let digest_a = [1u8; 32];
        let digest_b = [2u8; 32];

        let low = block_version_key(id, 1, &digest_a, None);
        let high = block_version_key(id, 2, &digest_a, None);
        assert_eq!(low.len(), BLOCK_KEY_LEN);
        assert!(high > low);

        let tie_low = block_version_key(id, 1, &digest_a, None);
        let tie_high = block_version_key(id, 1, &digest_b, None);
        assert!(tie_high > tie_low);
    }

    #[test]
    fn previous_digest_zero_sentinel() {
        let mut suffix = [0u8; CRDT_SUFFIX_LEN];
        CrdtKeySuffix::write_into(&mut suffix, 1, &[9u8; 32], None);
        assert_eq!(CrdtKeySuffix::read_previous_digest(&suffix), None);

        let prev = [7u8; 32];
        CrdtKeySuffix::write_into(&mut suffix, 1, &[9u8; 32], Some(&prev));
        assert_eq!(
            CrdtKeySuffix::read_previous_digest(&suffix),
            Some(prev)
        );
    }
}
