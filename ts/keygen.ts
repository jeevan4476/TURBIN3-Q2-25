import { Keypair } from "@solana/web3.js";
import promptSync from "prompt-sync";
import bs58 from "bs58";

let kp = Keypair.generate();
console.log(`You've generated a new Solana wallet:${kp.publicKey.toBase58()}`);

console.log(`[${kp.secretKey}]`)


const promptInput = promptSync();

console.log("++++++++++++ Welcome to TypeScript CLI ++++++++++++");

while (true) {
  console.log("\n1. Convert Public Key (Base58) to Uint8Array");
  console.log("2. Convert Uint8Array (JSON) to Base58");
  console.log("3. Exit");

  const value = promptInput("Enter the number: ").trim();

  if (value === "1") {
    const publickey = promptInput("Paste your public key here: ").trim();
    try {
      const byteArray = bs58.decode(publickey);
      console.log("The Uint8Array of your public key is:", byteArray);
    } catch (error) {
      console.error("Invalid Public Key! Make sure it's in Base58 format.");
    }
    continue;
  } else if (value === "2") {
    const privateKeyInput = promptInput("Paste your private key (JSON Array) here: ").trim();
    try {
      const jsonPrivateKey = JSON.parse(privateKeyInput);
      if (!Array.isArray(jsonPrivateKey)) throw new Error("Invalid private key format!");
      const base58Encoded = bs58.encode(new Uint8Array(jsonPrivateKey));
      console.log("The Base58-encoded private key is:", base58Encoded);
    } catch (error) {
      console.error("Invalid Private Key! Make sure it's a JSON Array.");
    }
    continue;
  } else if (value === "3") {
    console.log("Exiting...");
    break;
  } else {
    console.log("Enter a valid number (1, 2, or 3).");
  }
}