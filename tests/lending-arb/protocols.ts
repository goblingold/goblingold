import { Protocols } from "goblin-sdk-local";

export function getProtocols(token: string): Protocols[] {
  switch (token) {
    case "USDC":
      return [Protocols.Solend, Protocols.Francium];
    default:
      throw new Error("Invalidid asset '" + token + "':");
  }
}
