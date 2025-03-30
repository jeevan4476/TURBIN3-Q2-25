mod programs;
#[cfg(test)]
mod tests {
    use solana_sdk::{ message::Message, signature::{read_keypair_file, Keypair, Signer}, signer::keypair, system_program, transaction::Transaction};
    use bs58;
    use std::{io::{self,BufRead},str::FromStr};
    use solana_client::rpc_client::RpcClient;
    use solana_program::{
        hash::hash, pubkey::{self, Pubkey}, system_instruction::transfer
    };
    use crate::programs::Turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs,
        UpdateArgs};

    const RPC_URL :&str = "https://api.devnet.solana.com";
     #[test]
     fn keygen(){
         let kp = Keypair::new();
         println!("You've generated a new Solana wallet: {}",kp.pubkey().to_string());
         println!("");
         println!("To save your wallet, copy and paste the following into a JSON file:");
         println!("{:?}", kp.to_bytes());
     }

     #[test]
     fn base58_to_wallet(){
         let stdin = io::stdin();
         let base58 = stdin.lock().lines().next().unwrap().unwrap();
         println!("You wallet file is:");
         let wallet = bs58::decode(base58).into_vec().unwrap();
         println!("{:?}",wallet);
     }

     #[test]
     fn wallet_to_base58(){
         println!("Input your private key as a wallet file byte array:");
         let stdin = io::stdin();
         let wallet = stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').
             split(',') .map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();
         println!("Your private key is:");
         let base58 = bs58::encode(wallet).into_string();
         println!("{:?}",base58);
     }

    #[test]
     fn airdrop(){
         let keypair = read_keypair_file("dev-wallet.json").expect("couldn't find wallet file");
         let client = RpcClient::new(RPC_URL);
         match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
             Ok(s)=>{
                 println!("Success! Check out your TX here:");
                 println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
             },
             Err(e)=>{
                 println!("Oops, something went wrong: {}", e.to_string())
             }
         }

     }

    #[test]
    fn tranfer_sol(){
        let keypair= read_keypair_file("dev-wallet.json").expect("Couldn't find the wallet file");
        let pubkey = keypair.pubkey();
        let message_bytes=b"I verify my solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        let sig_hashed =  hash(sig.as_ref());
        match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()){
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }

        let to_pubkey = Pubkey::from_str("G8QCPDjj6DTgaJxzWiGumfz7dXA839UQqgXHbBssA3A3").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);

        let recent_blockchash = rpc_client.get_latest_blockhash().expect("failed to get latest blockhash");

        let transaction = Transaction::new_signed_with_payer(&[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)], Some(&keypair.pubkey()), &vec![&keypair], recent_blockchash);
        let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature);


        let balance = rpc_client.get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        let message = Message::new_with_blockhash(&[transfer(&keypair.pubkey(), &to_pubkey, balance)], Some(&keypair.pubkey()), &recent_blockchash);
        let fee = rpc_client.get_fee_for_message(&message).expect("Failed to get fee calculates");
        let transaction = Transaction::new_signed_with_payer(&[transfer(&keypair.pubkey(), &to_pubkey, balance-fee)], Some(&keypair.pubkey()), &vec![&keypair], recent_blockchash);
        let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature);


    }

    #[test]
    fn enroll(){
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldnt find wallet file");

        let prereq = Turbin3PrereqProgram::derive_program_address(&[b"prereq",signer.pubkey().to_bytes().as_ref()]);
        let args = CompleteArgs{
            github: b"jeevan4476".to_vec()
        };
        let blockhash = rpc_client .get_latest_blockhash() .expect("Failed to get recent blockhash");

        let transaction =Turbin3PrereqProgram::complete(&[&signer.pubkey(),&prereq,&system_program::id()], &args, Some(&signer.pubkey()), &[&signer], blockhash);
        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",signature);
    }
}
