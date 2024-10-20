const {
    Connection,
    PublicKey,
    Keypair,
    clusterApiUrl,
    Transaction,
    SystemProgram,
    sendAndConfirmTransaction,
} = require('@solana/web3.js');
const {
    Metadata,
    UpdateMetadataArgs,
    createUpdateMetadataInstruction,
} = require('@metaplex-foundation/js');
const { Token } = require('@solana/spl-token');

// Define the network (mainnet, devnet, etc.)
const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

// Replace with your token mint address
const mintPublicKey = new PublicKey("YOUR_MINT_ADDRESS");

// Replace with the update authority's keypair
const updateAuthority = Keypair.fromSecretKey(Uint8Array.from([""]));

// Fetch the associated metadata address for the token mint
async function findMetadataAddress(mintPublicKey) {
    return await PublicKey.findProgramAddress(
        [
            Buffer.from("metadata"),
            Metadata.programId.toBuffer(),
            mintPublicKey.toBuffer(),
        ],
        Metadata.programId
    );
}

// Create and send the transaction to update metadata
async function updateTokenMetadata() {
    // Get the metadata address
    const [metadataAddress] = await findMetadataAddress(mintPublicKey);

    // Define the updated metadata fields
    const updateMetadataArgs = new UpdateMetadataArgs({
        updateAuthority: updateAuthority.publicKey,  // Authority to update metadata
        data: {
            name: "New Token Name",  // Optional: Update token name
            symbol: "NEW",  // Optional: Update symbol
            uri: "https://new-uri-link.com/metadata.json",  // New metadata URI
            sellerFeeBasisPoints: 500,  // Optional: 5% seller fee, if NFT
            creators: null,  // Optional: Set creators if relevant
        },
        primarySaleHappened: null,  // Optional
        isMutable: null,  // Optional: Indicate whether the metadata is mutable
    });

    // Create the instruction to update metadata
    const instruction = createUpdateMetadataInstruction({
        metadata: metadataAddress,
        updateAuthority: updateAuthority.publicKey,
        args: updateMetadataArgs,
    });

    // Create and sign the transaction
    const transaction = new Transaction().add(instruction);
    const signature = await sendAndConfirmTransaction(
        connection,
        transaction,
        [updateAuthority]
    );

    console.log("Transaction Signature: ", signature);
}

updateTokenMetadata()
    .then(() => console.log('Metadata updated successfully'))
    .catch(console.error);