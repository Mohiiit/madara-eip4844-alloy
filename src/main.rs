//! Example of using the HTTP provider to get the latest block number.

use alloy_consensus::{
    BlobTransactionSidecar as consensusBlobTransactionSidecar, SidecarBuilder, SignableTransaction,
    TxEip4844, TxEip4844Variant, TxEip4844WithSidecar, TxEnvelope,
};
use alloy_eips::eip2718::Encodable2718;
use alloy_eips::eip2930::AccessListItem;
use alloy_network::{Ethereum, EthereumSigner, TxSignerSync};
use alloy_node_bindings::Anvil;
use alloy_primitives::{address, bytes, Bytes, FixedBytes, U256, U8};
use alloy_provider::{HttpProvider, Provider, ProviderBuilder};
use alloy_rlp::{length_of_length, BufMut, Decodable, Encodable, Header, Rlp, RlpEncodable};
use alloy_rpc_client::RpcClient;
use alloy_rpc_types::{request::TransactionRequest, BlobTransactionSidecar, TransactionInput};
use alloy_signer_wallet::{LocalWallet, Wallet};
use eyre::Result;
// use kzg::{KzgCommitment, KzgProof, KzgSettings};
// use alloy_eips::eip4844::MAINNET_KZG_TRUSTED_SETUP;

// pub fn generate_blob_sidecar(blobs: Vec<Blob>) -> BlobTransactionSidecar {
//     let kzg_settings = MAINNET_KZG_TRUSTED_SETUP.clone();

//     let commitments: Vec<Bytes48> = blobs
//         .iter()
//         .map(|blob| KzgCommitment::blob_to_kzg_commitment(&blob.clone(), &kzg_settings).unwrap())
//         .map(|commitment| commitment.to_bytes())
//         .collect();

//     let proofs: Vec<Bytes48> = blobs
//         .iter()
//         .zip(commitments.iter())
//         .map(|(blob, commitment)| {
//             KzgProof::compute_blob_kzg_proof(blob, commitment, &kzg_settings).unwrap()
//         })
//         .map(|proof| proof.to_bytes())
//         .collect();

//     BlobTransactionSidecar {
//         blobs,
//         commitments,
//         proofs,
//     }
// }
// async fn send_blob_txns_v3() -> Result<()> {
//     let rpc_url = "https://sepolia.infura.io/v3/3753abe17c124d088f4e68d58f257452".parse()?;

//     // Create the RPC client.
//     let rpc_client = RpcClient::new_http(rpc_url);

//     // Provider can then be instantiated using the RPC client, HttpProvider is an alias
//     // RootProvider. RootProvider requires two generics N: Network and T: Transport
//     let provider = HttpProvider::<Ethereum>::new(rpc_client);
//     let wallet = "cf43d32585172619dbc5b46b20560e96c563c01f0e55e1b36837d8441ca36a8a"
//         .parse::<LocalWallet>()?;
//     let nonce = provider
//         .get_transaction_count(wallet.address(), None)
//         .await?;
//     let estimation = provider.estimate_eip1559_fees(None).await?;
//     let mut builder: SidecarBuilder = SidecarBuilder::new();
//     // ingest block data
//     builder.ingest(b"dummy blob");
//     // build the sidecar with default KZG settings after all ingestion is finished
//     let _sidecar = builder.build()?;

//     let fixed_bytes_vec_blob_hashes: Vec<FixedBytes<32>> = vec![
//         FixedBytes([0u8; 32]), // One FixedBytes element filled with zeros
//         FixedBytes([7u8; 32]), // Another element filled with the byte 7
//     ];
//     // let first_access_item: AccessListItem = AccessListItem {
//     //     address: bob,
//     //     storage_keys: fixed_bytes_vec_blob_hashes,
//     // };
//     // let access_list: alloy_eips::eip2930::AccessList =
//     // alloy_eips::eip2930::AccessList(vec![first_access_item]);
//     let mut tx = TxEip4844 {
//         chain_id: 1,
//         nonce: nonce.to_string().parse().unwrap(),
//         gas_limit: 2_000_000,
//         max_fee_per_gas: estimation.max_fee_per_gas.to_string().parse().unwrap(),
//         max_priority_fee_per_gas: estimation
//             .max_priority_fee_per_gas
//             .to_string()
//             .parse()
//             .unwrap(),
//         to: address!("40B5d89fCE766a0076Bd55dE77726696527e06B7").into(),
//         value: U256::from(0),
//         access_list: Default::default(),
//         blob_versioned_hashes: fixed_bytes_vec_blob_hashes,
//         max_fee_per_blob_gas: 65_535,
//         input: bytes!(),
//     };
//     let signature = wallet.sign_transaction_sync(&mut tx)?;
//     let tx2 = TxEip4844WithSidecar::from_tx_and_sidecar(tx, _sidecar);
//     let variant = TxEip4844Variant::TxEip4844WithSidecar(tx2);
//     let signed = variant.into_signed(signature);
//     let envelope = TxEnvelope::Eip4844(signed);
//     let result = provider
//         .send_raw_transaction(envelope.encoded_2718().into())
//         .await?;
//     Ok(())
// }
// async fn send_blob_txns_v2() -> Result<()> {
//     let anvil = Anvil::new().fork("https://eth.merkle.io").try_spawn()?;
//     let rpc_url = anvil.endpoint().parse()?;
//     // let provider = HttpProvider::<Ethereum>::new_http(rpc_url);
//     let signer: LocalWallet = anvil.keys()[0].clone().into();
//     // let wallet = signer.parse::<LocalWallet>()?;
//     // let wallet = wallet.chain_id(Some(1));
//     let provider = HttpProvider::<Ethereum>::new_http(rpc_url); // let provider = ProviderBuilder::new()
//                                                                 //     .signer(EthereumSigner::from(signer))
//                                                                 //     .on_client(RpcClient::new_http(rpc_url));
//     let _alice = anvil.addresses()[0];
//     let bob = anvil.addresses()[1];
//     let nonce = provider.get_transaction_count(_alice, None).await?;
//     let estimation = provider.estimate_eip1559_fees(None).await?;
//     // let chain_id: u64 = provider.get_chain_id().await?;
//     let fixed_bytes_vec_blob_hashes: Vec<FixedBytes<32>> = vec![
//         FixedBytes([0u8; 32]), // One FixedBytes element filled with zeros
//         FixedBytes([7u8; 32]), // Another element filled with the byte 7
//     ];
//     // let first_access_item: AccessListItem = AccessListItem {
//     //     address: bob,
//     //     storage_keys: fixed_bytes_vec_blob_hashes,
//     // };
//     // let access_list: alloy_eips::eip2930::AccessList =
//     // alloy_eips::eip2930::AccessList(vec![first_access_item]);
//     let mut tx = TxEip4844 {
//         chain_id: 1,
//         nonce: nonce.to_string().parse().unwrap(),
//         gas_limit: 2_000_000,
//         max_fee_per_gas: estimation.max_fee_per_gas.to_string().parse().unwrap(),
//         max_priority_fee_per_gas: estimation
//             .max_priority_fee_per_gas
//             .to_string()
//             .parse()
//             .unwrap(),
//         to: bob,
//         value: U256::from(0),
//         access_list: Default::default(),
//         blob_versioned_hashes: fixed_bytes_vec_blob_hashes,
//         max_fee_per_blob_gas: 65_535,
//         input: bytes!(),
//     };

//     let mut builder: SidecarBuilder = SidecarBuilder::new();
//     // ingest block data
//     builder.ingest(b"dummy blob");
//     // build the sidecar with default KZG settings after all ingestion is finished
//     let _sidecar = builder.build()?;

//     let signature = signer.sign_transaction_sync(&mut tx)?;
//     let tx2 = TxEip4844WithSidecar::from_tx_and_sidecar(tx, _sidecar);
//     let variant = TxEip4844Variant::TxEip4844WithSidecar(tx2);
//     let signed = variant.into_signed(signature);
//     let envelope = TxEnvelope::Eip4844(signed);

//     Ok(())
// }

async fn send_blob_txns() -> Result<()> {
    let anvil = Anvil::new().fork("https://eth.merkle.io").try_spawn()?;

    // Create a provider.
    let rpc_url = anvil.endpoint().parse()?;
    let provider = HttpProvider::<Ethereum>::new_http(rpc_url);

    // Create two users, Alice and Bob.
    let _alice = anvil.addresses()[0];
    let bob = anvil.addresses()[1];

    let mut builder: SidecarBuilder = SidecarBuilder::new();
    // ingest block data
    builder.ingest(b"dummy blob");
    // build the sidecar with default KZG settings after all ingestion is finished
    let _sidecar = builder.build()?;
    let fixed_bytes_vec_blob: Vec<FixedBytes<131072>> = vec![
        FixedBytes([0u8; 131072]), // One FixedBytes element filled with zeros
        FixedBytes([7u8; 131072]), // Another element filled with the byte 7
    ];
    let fixed_bytes_vec_commitments: Vec<FixedBytes<48>> = vec![
        FixedBytes([0u8; 48]), // One FixedBytes element filled with zeros
        FixedBytes([7u8; 48]), // Another element filled with the byte 7
    ];
    let fixed_bytes_vec_proofs: Vec<FixedBytes<48>> = vec![
        FixedBytes([0u8; 48]), // One FixedBytes element filled with zeros
        FixedBytes([7u8; 48]), // Another element filled with the byte 7
    ];
    let right_sidecar = BlobTransactionSidecar {
        blobs: fixed_bytes_vec_blob,
        commitments: fixed_bytes_vec_commitments,
        proofs: fixed_bytes_vec_proofs,
    };
    // Create a transaction to transfer 1 wei from Alice to Bob.
    // let data = BlobReader::readBlobCall::new(()).abi_encode();
    // let current_dir = std::env::current_dir()?;
    // let file_path = current_dir.join("src/keystore.json");
    // println!("file path {}", file_path.display());
    // let json_content = fs::read_to_string(file_path).expect("Failed to read the blob data file");
    // let json_value: serde_json::Value =
    //     serde_json::from_str(&json_content).expect("Failed to deserialize JSON");
    // let blobs: Vec<Blob> = vec![Blob::from_hex(
    //     json_value
    //         .get("data")
    //         .unwrap()
    //         .as_str()
    //         .expect("Data is not a valid string"),
    // )
    // .unwrap()];
    // let sidecar: BlobTransactionSidecar = generate_blob_sidecar(blobs);
    // let blob_sidecar = BlobTransactionSidecar {
    //     blobs: sidecar
    //         .blobs
    //         .into_iter()
    //         .map(|b| c_kzg::Blob::from_bytes(b.as_slice()).unwrap())
    //         .collect(),
    //     commitments: sidecar
    //         .commitments
    //         .into_iter()
    //         .map(|c| c_kzg::Bytes48::from_bytes(c.as_slice()).unwrap())
    //         .collect(),
    //     proofs: sidecar
    //         .proofs
    //         .into_iter()
    //         .map(|p| c_kzg::Bytes48::from_bytes(p.as_slice()).unwrap())
    //         .collect(),
    // };
    let input_data = Bytes::from("some_input_data");
    // let transaction_type = Some(U8::new(3));
    let transaction_type: U8 = "3".parse().unwrap();

    let tx: TransactionRequest = TransactionRequest {
        to: Some(bob),
        input: TransactionInput::new(input_data),
        transaction_type: Some(transaction_type),
        sidecar: Some(right_sidecar),
        blob_versioned_hashes: Default::default(),
        max_fee_per_blob_gas: Default::default(),
        ..Default::default()
    };
    // // let tx = TransactionRequest::default()
    // //     .from(alice)
    // //     .value(U256::from(1))
    // //     .to(Some(bob));
    print!("something");
    // // Send the transaction and wait for the receipt.
    let pending_tx = provider.send_transaction(tx).await?;

    println!("Pending transaction...{:?}", pending_tx.tx_hash());

    // // Wait for the transaction to be included.
    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block: {:?}",
        receipt
            .block_number
            .expect("Failed to get block number")
            .to_string()
    );

    // // assert_eq!(receipt.from, alice);
    // assert_eq!(receipt.to, Some(bob));

    Ok(())
}
async fn send_simple_txns() -> Result<()> {
    let anvil = Anvil::new().fork("https://eth.merkle.io").try_spawn()?;

    // Create a provider.
    let rpc_url = anvil.endpoint().parse()?;
    let provider = HttpProvider::<Ethereum>::new_http(rpc_url);

    // Create two users, Alice and Bob.
    let alice = anvil.addresses()[0];
    let bob = anvil.addresses()[1];

    // Create a transaction to transfer 1 wei from Alice to Bob.
    let tx = TransactionRequest::default()
        .from(alice)
        .value(U256::from(1))
        .to(Some(bob));

    // Send the transaction and wait for the receipt.
    let pending_tx = provider.send_transaction(tx).await?;
    let chain_id = provider.get_chain_id().await?;

    println!("chain id here is {}", chain_id);

    println!("Pending transaction...{:?}", pending_tx.tx_hash());

    // Wait for the transaction to be included.
    let receipt = pending_tx.get_receipt().await?;

    println!(
        "Transaction included in block: {:?}",
        receipt
            .block_number
            .expect("Failed to get block number")
            .to_string()
    );

    assert_eq!(receipt.from, alice);
    assert_eq!(receipt.to, Some(bob));

    Ok(())
}
#[tokio::main]
async fn main() -> Result<()> {
    send_simple_txns().await?;
    send_blob_txns().await?;
    // send_blob_txns_v2().await?;

    Ok(())
}
