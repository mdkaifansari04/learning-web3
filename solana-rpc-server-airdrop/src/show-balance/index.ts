import { PublicKey, Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";

export const showBalance = async (publicKey: PublicKey) => {
    const connection = new Connection("https://api.devnet.solana.com");
    const balance = await connection.getBalance(publicKey);
    return balance / LAMPORTS_PER_SOL;
}

(async () => {
    const publicKey = new PublicKey("FfL4LhBP65krJHD4gtPYFsEgpbC2udWqJ3QL8p6JAV3U");
    const balance = await showBalance(publicKey);
    console.log(`Balance of this address ${publicKey.toBase58()} is ${balance} SOL`);
})();