
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
    "0x670039c117d738848ff6cbfe113629fda8c5c5e8275298116f04035016814229"
        .parse()
        .expect("Invalid ID");

        let sub_id_array = [0u8; 32];
        let sub_id = Bits256(sub_id_array);

    let contract_instance = MyContract::new(contract_id, wallet.clone());
    let asset_id = contract_instance.methods().asset_id(sub_id).call().await.unwrap();
    let asset = asset_id.value;
    println!("asset {:?}",asset);
    let form = Identity::Address(wallet.address().into());
    let init = contract_instance.methods().constructor(form).call().await.unwrap();
    dbg!(&init);
   
   
}