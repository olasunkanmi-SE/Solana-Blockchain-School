import { Keypair } from "@solana/web3.js";
import { Buffer } from "buffer";

export const generateKeyPair = (): Keypair => {
  return Keypair.generate();
};

export const secretKeyInHex = (Keypair: Keypair) => {
  return Buffer.from(Keypair.secretKey).toString("hex");
};

export const generateKeyPairFromSecretKey = (secretKeyInHex: string) => {
  const unsignedSecretKey = Uint8Array.from(Buffer.from(secretKeyInHex, "hex"));
  return Keypair.fromSecretKey(unsignedSecretKey);
};
