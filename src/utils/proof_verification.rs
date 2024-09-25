use crate::rpc::proofs::StorageProof;
use blake2::{Blake2s256, Digest};
use anyhow::Result;
use hex;

fn compute_key(account_address: &str, storage_key: &str) -> Result<Vec<u8>> {
    let account_address_bytes = hex::decode(account_address.trim_start_matches("0x"))?;
    let storage_key_bytes = hex::decode(storage_key.trim_start_matches("0x"))?;
    let mut data = vec![0_u8; 12];
    data.extend_from_slice(&account_address_bytes);
    data.extend_from_slice(&storage_key_bytes);

    println!("Computing Key:");
    println!("  Data to Hash: 0x{}", hex::encode(&data));

    // Compute blake2s256 hash
    let mut hasher = Blake2s256::new();
    hasher.update(&data);
    let key = hasher.finalize().to_vec();

    println!("  Key: 0x{}", hex::encode(&key));

    Ok(key)
}


fn get_key_bits(key: &[u8]) -> Vec<bool> {
    let mut bits = Vec::with_capacity(256);
    for byte in key.iter().rev() {
        for i in 0..8 {
            bits.push((byte >> i) & 1 != 0);
        }
    }
    bits
}


pub fn verify_proof(
    proof: &StorageProof,
    root_hash: &str,
    account_address: &str,
    storage_key: &str,
) -> Result<bool> {
    let mut current_hash = compute_leaf_hash(proof)?;
    println!("Leaf Hash: 0x{}", hex::encode(&current_hash));

    // Compute the key used in the Merkle tree
    let key = compute_key(account_address, storage_key)?;
    let key_bits: Vec<bool> = get_key_bits(&key);

    // Process proof hashes in root-to-leaf order
    let proof_hashes: Vec<&String> = proof.proof.iter().collect();

    println!("Key Bits (MSB to LSB):");
    for (i, bit) in key_bits.iter().enumerate() {
        print!("{}", if *bit { 1 } else { 0 });
        if (i + 1) % 8 == 0 {
            print!(" ");
        }
    }
    println!();

    for (depth, sibling_hash_hex) in proof_hashes.iter().enumerate() {
        let sibling_hash = hex::decode(sibling_hash_hex.trim_start_matches("0x"))?;
        let mut hasher = Blake2s256::new();

        let bit = if depth < key_bits.len() {
            key_bits[depth]
        } else {
            false
        };

        println!("Level {}:", depth);
        println!("  Bit [{}]: {}", depth, if bit { 1 } else { 0 });
        println!("  Sibling Hash: 0x{}", sibling_hash_hex);
        
        if bit {
            // bit is 1, current node is right child
            hasher.update(&current_hash); // Left child
            hasher.update(&sibling_hash); // Right child
            println!("  Position: Right Child");
        } else {
            // bit is 0, current node is left child
            hasher.update(&sibling_hash); // Left child
            hasher.update(&current_hash); // Right child
            println!("  Position: Left Child");
        }

        current_hash = hasher.finalize_reset().to_vec();
        println!("  Combined Hash: 0x{}", hex::encode(&current_hash));
    }

    let reconstructed_root_hash = format!("0x{}", hex::encode(&current_hash));
    println!("Reconstructed Root Hash: {}", reconstructed_root_hash);
    println!("Expected Root Hash: {}", root_hash);

    Ok(reconstructed_root_hash == root_hash)
}



fn compute_leaf_hash(proof: &StorageProof) -> Result<Vec<u8>> {
    let index_bytes = (proof.index as u64).to_be_bytes(); // Big-endian 8 bytes
    let value_bytes = hex::decode(proof.value.trim_start_matches("0x"))?;

    println!("Computing Leaf Hash:");
    println!("  Index Bytes: 0x{}", hex::encode(&index_bytes));
    println!("  Value Bytes: 0x{}", hex::encode(&value_bytes));

    let leaf_value = [index_bytes.as_ref(), value_bytes.as_ref()].concat();

    let mut hasher = Blake2s256::new();
    hasher.update(&leaf_value);
    let leaf_hash = hasher.finalize().to_vec();

    println!("  Leaf Value: 0x{}", hex::encode(&leaf_value));
    println!("  Leaf Hash: 0x{}", hex::encode(&leaf_hash));

    Ok(leaf_hash)
}




