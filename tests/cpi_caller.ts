// tests/multi_wallet_swap_full.ts
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MultiWalletSwap } from "../target/types/multi_wallet_swap";
import { PublicKey, SystemProgram, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";

// Helper to airdrop SOL
async function airdropSol(provider: anchor.AnchorProvider, pubkey: PublicKey, amount: number) {
    const sig = await provider.connection.requestAirdrop(pubkey, amount);
    await provider.connection.confirmTransaction(sig);
}

describe("multi_wallet_swap", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.MultiWalletSwap as Program<MultiWalletSwap>;

    const user = Keypair.generate();
    const feeReceiver = Keypair.generate();

    // Replace these with actual account addresses for real test
    const pool = new PublicKey("A9YY4epyvVCuDKtTvN1Ethbhcnvv5fkodUyQa1E8nPqL");
    const userBaseAccount = new PublicKey("8UpJ2DJyusitGLFJV2KakArJYbKxvgUf5RfG6VjP2jbW");
    const userQuoteAccount = new PublicKey("HDfBPEgpxp6Ry6nsdtes6noz7U8bzC725cV1c2CrAYNQ");
    const poolBaseAccount = new PublicKey("FiHgWYHs7L5Krt8rctWMi6JcrFeD6k1P8gHFDruMyqB");
    const poolQuoteAccount = new PublicKey("BfPr7ZcKHSrbxedtcNs9NFoHh6p3LBxd7yvukdrYX4dR");
    const protocolFeeRecipient = new PublicKey("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV");
    const eventAuthority = new PublicKey("GS4CU59F31iL7aR2Q8zVS8DRrcRnXX1yjQ66TqNVQnaR");
    const pumpSwapProgram = new PublicKey("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA");
    const globalConfig = new PublicKey("ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw");
    const baseMint = new PublicKey("FQb4D4kSe3DcwMd8khvaYBzTgwFXWsfsL5ZP5mWepump");
    const quoteMint = new PublicKey("So11111111111111111111111111111111111111112");
    const protocolFeeRecipientTokenAccount = new PublicKey("94qWNrtmfn42h3ZjUZwWvK1MEo9uVmmrBPd2hpNjYDjb");
    const associatedTokenProgram = new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

    before(async () => {
        await airdropSol(provider, user.publicKey, 2 * LAMPORTS_PER_SOL);
        await airdropSol(provider, feeReceiver.publicKey, LAMPORTS_PER_SOL);
    });

    it("Performs a mocked CPI swap with fee collection", async () => {
        const swapInput1 = {
            baseAmountOut: new anchor.BN(1000000),
            maxQuoteAmountIn: new anchor.BN(1200000),
        };
        const swapInput2 = {
            baseAmountOut: new anchor.BN(3000000),
            maxQuoteAmountIn: new anchor.BN(1200000),
        };

        const feeBps = 100; // 1% fee

        const tx = await program.methods
            .multiWalletSwap([swapInput1, swapInput2], feeBps)
            .accounts({
                user: user.publicKey,
                feeReceiver: feeReceiver.publicKey,
                pool,
                userBaseAccount,
                userQuoteAccount,
                poolBaseAccount,
                poolQuoteAccount,
                protocolFeeRecipient,
                baseTokenProgram:new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                quoteTokenProgram:new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                eventAuthority,
                pumpSwapProgram,
                globalConfig,
                baseMint,
                quoteMint,
                protocolFeeRecipientTokenAccount,
                associatedTokenProgram,

            })
            .signers([user])
            .rpc();

        console.log("Simulated tx signature:", tx);
    });
});