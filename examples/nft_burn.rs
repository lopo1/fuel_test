
#[warn(unused_imports)]
use fuels::{crypto::SecretKey, prelude::*, types::{Bits256,Bytes32, ContractId, Identity}};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    abigen!(Contract(
        name = "MyContract",
        abi = "out/debug/src7_token-abi.json"
    ));
    let provider = Provider::connect("https://testnet.fuel.network/v1/graphql").await.unwrap();
    let secret =
        SecretKey::from_str("cc74e38f91112c34aa913592888e3a582850bfeaaa04cbb6914a746a9eb564e1")
            .unwrap();

    // Create the wallet
    let wallet = WalletUnlocked::new_from_private_key(secret, Some(provider.clone()));

    let contract_id: ContractId  =
        "0x20667443791e287f39703462e179904ba106063fd7c16c5188e9a5ba9e6faa78"
            .parse()
            .expect("Invalid ID");

    let hex_str = format!("{:064x}", 1);

    // 添加 '0x' 前缀
    let hex_str_with_prefix = format!("0x{}", hex_str);
    println!("hex_str_with_prefix {}",hex_str_with_prefix);
    let sub_id = Bits256::from_hex_str(&hex_str_with_prefix).unwrap();


    let contract_instance = MyContract::new(contract_id, wallet.clone());
    let asset_id = contract_instance.methods().asset_id(sub_id).call().await.unwrap();
    let asset = asset_id.value;
    println!("asset {:?}",asset);
    let form = Identity::Address(wallet.address().into());
    let init = contract_instance.methods().constructor(form).call().await.unwrap();
    // 将数字转换为十六进制字符串
    let burn = contract_instance.methods().burn(sub_id,1).with_variable_output_policy(VariableOutputPolicy::EstimateMinimum).with_tx_policies(TxPolicies::default()).call().await;

    dbg!(&burn);


}