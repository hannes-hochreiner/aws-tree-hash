use sha2::{Digest, Sha256};

const BLOCK_SIZE: usize = 1024 * 1024;

/// Calculate the Amazon SHA256 tree hash as described on [Checksum Calculation page](https://docs.aws.amazon.com/amazonglacier/latest/dev/checksum-calculations.html) of the AWS S3 Glacier developer guide.
/// The function is meant to be used on the complete data in the case of an upload in a single request and on each part in case of a multi-part upload.
pub fn calculate_tree_hash(data: &[u8]) -> Vec<u8> {
    let mut start = 0;
    let mut end = start + BLOCK_SIZE;
    let mut hashes = Vec::new();

    while end < data.len() {
        let mut hasher = Sha256::new();

        hasher.update(&data[start..end]);
        hashes.push(hasher.finalize().to_vec());

        start = end;
        end += BLOCK_SIZE;
    }

    let mut hasher = Sha256::new();

    hasher.update(&data[start..data.len()]);
    hashes.push(hasher.finalize().to_vec());

    while hashes.len() > 1 {
        hashes = combine_hashes(hashes);
    }

    hashes[0].clone()
}

/// Combine the tree hashes from multiple parts (i.e. multiple invocations of [`calculate_tree_hash`]).
///
/// [`calculate_tree_hash`]: fn.calculate_tree_hash.html
pub fn combine_hashes(hashes: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut iter = hashes.iter();
    let mut vec = Vec::new();

    loop {
        match (iter.next(), iter.next()) {
            (Some(e1), Some(e2)) => {
                let mut combined = e1.to_owned();

                combined.extend(e2);

                let mut hasher = Sha256::new();

                hasher.update(combined);
                vec.push(hasher.finalize().to_vec());
            }
            (Some(e1), None) => vec.push(e1.to_owned()),
            (_, _) => break,
        }
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::FromHex;

    #[test]
    fn null_data() {
        assert_eq!(
            calculate_tree_hash(&[0; 7680000]),
            Vec::from_hex("7a43777ddc7a0326d36b15bc482e6c7736e1c2e9d80a647e8c301646f6a4785c")
                .unwrap()
        );
    }
}
