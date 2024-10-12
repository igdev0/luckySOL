use rs_merkle::{algorithms::Sha256, Hasher, MerkleProof, MerkleTree};

fn main() {
    let leaf_values = ["a", "b", "c", "d", "e", "f"];

    // Hash each leaf
    let leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| Sha256::hash(x.as_bytes()))
        .collect();

    // Create merkle tree from leaves
    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);

    // Prove these two leafes exist on the merkle tree
    let indices_to_prove = vec![3, 4];
    let leaves_to_prove = leaves.get(3..5).ok_or("can't get leaves to prove").unwrap();
    let merkle_proof = merkle_tree.proof(&indices_to_prove);
    let merkle_root = merkle_tree
        .root()
        .ok_or("couldn't get the merkle root")
        .unwrap();
    // Serialize proof to pass it to the client
    let proof_bytes = merkle_proof.to_bytes();

    // Parse proof back on the client
    let proof = MerkleProof::<Sha256>::try_from(proof_bytes).unwrap();
    let verification_result = proof.verify(
        merkle_root,
        &indices_to_prove,
        leaves_to_prove,
        leaves.len(),
    );
    assert!(verification_result);
}
