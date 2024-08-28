import { useContext } from "react";
import { ConnectionContext, IConnectionContextValue } from "./connectionProvider";

export const useConnectionContext = (): IConnectionContextValue => {
  return useContext(ConnectionContext);
};
