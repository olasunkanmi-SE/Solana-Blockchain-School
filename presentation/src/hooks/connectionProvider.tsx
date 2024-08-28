import { Connection } from "@solana/web3.js";
import { createContext, useEffect, useMemo, useState } from "react";

export interface IConnectionContextValue {
  connection: Connection | null;
}

export const ConnectionContext: React.Context<IConnectionContextValue> = createContext<IConnectionContextValue>({
  connection: null,
});

export interface IConnectionContextProps {
  children: React.ReactNode;
}

export const ConnectionProvider = ({ children }: IConnectionContextProps) => {
  const [connection, setConnection] = useState<Connection | null>(null);

  useEffect(() => {
    const establishConnection = () => {
      const url = import.meta.env.VITE_CONNECTION_URL;
      const connection = new Connection(url, "confirmed");
      console.debug(`hello`);
      setConnection(connection);
    };
    establishConnection();
  }, []);

  const contextValue = useMemo(() => ({ connection }), [connection]);

  return <ConnectionContext.Provider value={contextValue}>{children}</ConnectionContext.Provider>;
};
