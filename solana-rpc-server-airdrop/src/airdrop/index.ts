import { Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

export const airDropSolana = async (address: string, amount: number) => {
  const publicKey = new PublicKey(address);
  const conn = new Connection("http://api.de:8899", "confirmed");
  const res = await conn.requestAirdrop(publicKey, amount * LAMPORTS_PER_SOL);

  console.log("res", res);
};

airDropSolana("FfL4LhBP65krJHD4gtPYFsEgpbC2udWqJ3QL8p6JAV3U", 1);
