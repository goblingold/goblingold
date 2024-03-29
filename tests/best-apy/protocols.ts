import { Protocols } from "goblin-sdk-local";

export function getProtocols(token: string): Protocols[] {
  switch (token) {
    case "BTC":
    case "SRM":
    case "WSOL":
      return [
        Protocols.Mango,
        Protocols.Solend,
        Protocols.Port,
        Protocols.Tulip,
        Protocols.Francium,
      ];

    case "MNGO":
      return [Protocols.Mango];

    case "ORCA":
      return [Protocols.Solend, Protocols.Tulip, Protocols.Francium];

    case "RAY":
      return [
        Protocols.Mango,
        Protocols.Solend,
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

    case "scnSOL":
      return [Protocols.Solend];

    case "soETH":
      return [
        Protocols.Mango,
        //Protocols.Solend "Reserve deposit limit
        Protocols.Tulip,
        Protocols.Francium,
      ];

    case "stSOL":
      return [
        Protocols.Solend,
        Protocols.Port,
        Protocols.Tulip,
        Protocols.Francium,
      ];

    case "SAMO":
      return [Protocols.Tulip, Protocols.Francium];

    case "ETH":
      return [
        Protocols.Francium,
        Protocols.Port,
        Protocols.Solend,
        Protocols.Tulip,
      ];

    default:
      throw new Error("Invalidid asset '" + token + "':");
  }
}
