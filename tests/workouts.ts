const assert = require('assert');
const anchor = require('@coral-xyz/anchor');
const { SystemProgram } = anchor.web3;

describe("workouts", () => {
    const provider = anchor.getProvider();
    anchor.setProvider(provider);
    const workouts = anchor.web3.Keypair.generate();
    const program = anchor.workspace.workouts;

    it('Is Initialized', async () => {
        await program.rpc.initialize("Welcome to Solana!", {
            accounts: {
                workouts: workouts.publicKey,
                user: provider.wallet.publicKey,
                systemProgram: SystemProgram.programId,
            },
            signers: [workouts]
        })
        const account = await program.account.workouts.fetch(workouts.publicKey);
       assert.ok(account.greeting = "LFG!!");    
    })

    it('Checks Workout Goals', async () => {
        await program.rpc.goal(new anchor.BN(100), {
            accounts: {
                workouts: workouts.publicKey,
            }
        })

        const account = await program.account.workouts.fetch(workouts.publicKey);
        assert.ok(account.remaining.eq(new anchor.BN(0)))
    })


});
