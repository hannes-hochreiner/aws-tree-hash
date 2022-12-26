# aws-tree-hash
A library for calculating AWS tree hashes

## Usage
### Single request upload
```rust
use hex::FromHex;

fn main() {
    assert_eq!(
        aws_tree_hash::calculate_tree_hash(&[0; 7680000]),
        Vec::from_hex("7a43777ddc7a0326d36b15bc482e6c7736e1c2e9d80a647e8c301646f6a4785c").unwrap()
    );
}
```

### Multi-part upload
```rust
use hex::FromHex;

fn main() {
    let data = &[0u8; 7680000];
    let tree_hash_1 = aws_tree_hash::calculate_tree_hash(&data[..2097152]);
    let tree_hash_2 = aws_tree_hash::calculate_tree_hash(&data[2097152..4194304]);
    let tree_hash_3 = aws_tree_hash::calculate_tree_hash(&data[4194304..6291456]);
    let tree_hash_4 = aws_tree_hash::calculate_tree_hash(&data[6291456..]);
    
    let res = aws_tree_hash::combine_hashes(vec![tree_hash_1, tree_hash_2, tree_hash_3, tree_hash_4]);
    
    assert_eq!(
        res,
        Vec::from_hex("7a43777ddc7a0326d36b15bc482e6c7736e1c2e9d80a647e8c301646f6a4785c")
            .unwrap()
    );
}
```

## Testing

The tree hash for a 7.7 MB file containing only null characters can be obtained by running `just src_other/start`.
The calculation is performed using the example algorithm provided by Amazon.

## License

This work is licensed under the MIT or Apache 2.0 license.

`SPDX-License-Identifier: MIT OR Apache-2.0`