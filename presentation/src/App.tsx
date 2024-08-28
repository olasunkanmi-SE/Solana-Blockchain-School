import { useEffect } from "react";
import "./App.css";
import reactLogo from "./assets/react.svg";
import { useConnectionContext } from "./hooks/connectionContext";
import { generateKeyPairFromSecretKey } from "./utils/utils";
import viteLogo from "/vite.svg";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

function App() {
  const { connection } = useConnectionContext();

  useEffect(() => {
    getWalletBallance();
  });

  const generateKeyPair = () => {
    const envSecret = import.meta.env.VITE_SECRET_KEY;
    return generateKeyPairFromSecretKey(envSecret);
  };

  const getWalletBallance = async (): Promise<number | undefined> => {
    const keyPair = generateKeyPair();
    const publicKey = keyPair.publicKey.toBase58();
    const address = new PublicKey(publicKey);
    const balance = await connection?.getBalance(address);
    console.log({ balance: balance ? balance / LAMPORTS_PER_SOL : 0 });
    return balance;
  };

  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>

      <p className="read-the-docs">Click on the Vite and React logos to learn more</p>
    </>
  );
}

export default App;
