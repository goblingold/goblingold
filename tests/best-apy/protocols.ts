import { Protocols } from "goblin-sdk-local";

export function getProtocols(token: string): Protocols[] {
  switch (token) {
    case "BTC":
    case "WSOL":
      return [
        Protocols.Mango,
        Protocols.Solend,
        Protocols.Port,
        Protocols.Tulip,
        Protocols.Francium,
      ];

    case "USDC":
    case "USDT":
      return [
        Protocols.Mango,
        Protocols.Solend,
        Protocols.Port,
        Protocols.Tulip,
        Protocols.Francium,
        Protocols.SolendStablePool,
      ];

    case "soETH":
      return [
        Protocols.Mango,
        //Protocols.Solend "Reserve deposit limit
        Protocols.Tulip,
        Protocols.Francium,
      ];

    default:
      throw new Error("Invalidid asset '" + token + "':");
  }
}
