import { Provider, Program } from '@project-serum/anchor';
import * as anchor from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';

// Import program ID and associated token program ID
const programID = new PublicKey('YourProgramID');
const tokenProgramID = new PublicKey('TokenProgramID');

// Initialize the provider
const provider = new anchor.Provider(
  new anchor.Wallet(provider.connection),
  {
    preflightCommitment: 'recent',
  },
);

// Initialize the program
const program = new Program(
  programID,
  programID,
  provider,
);

// Function to deposit SPL tokens
export const depositSPLToken = async (depositAmount: number) => {
  try {
    await program.rpc.depositSPLToken(depositAmount, {
      accounts: {
        user: provider.wallet.publicKey,
        account: program.account,
        mint: // Provide the mint account,
        authority: // Provide the authority account,
        splProgram: tokenProgramID,
        tokenProgram: tokenProgramID,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    });
    console.log('Deposit successful');
  } catch (error) {
    console.error('Error depositing SPL token:', error);
  }
};

// Function to deposit Solana lamports
export const depositSolanaLamports = async (depositAmount: number) => {
  try {
    await program.rpc.depositSolanaLamports(depositAmount, {
      accounts: {
        account: program.account,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      lamports: depositAmount,
    });
    console.log('Deposit successful');
  } catch (error) {
    console.error('Error depositing Solana lamports:', error);
  }
};

// Function to withdraw SPL tokens
export const withdrawSPLToken = async (withdrawAmount: number) => {
    try {
      await program.rpc.withdrawSPLToken(withdrawAmount, {
        accounts: {
          account: program.account,
          user: provider.wallet.publicKey,
          authority: // Provide the authority account,
          splProgram: tokenProgramID,
        },
      });
      console.log('Withdrawal successful');
    } catch (error) {
      console.error('Error withdrawing SPL token:', error);
    }
  };
  
  // Function to withdraw Solana lamports
  export const withdrawSolanaLamports = async (withdrawAmount: number) => {
    try {
      await program.rpc.withdrawSolanaLamports(withdrawAmount, {
        accounts: {
          account: program.account,
          user: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        lamports: withdrawAmount,
      });
      console.log('Withdrawal successful');
    } catch (error) {
      console.error('Error withdrawing Solana lamports:', error);
    }
  };
  
  // Function to transfer SPL tokens
  export const transferSPLTokens = async (amount: number, to: PublicKey) => {
    try {
      await program.rpc.transferSPLTokens(amount, to, {
        accounts: {
          account: program.account,
          to,
          authority: // Provide the authority account,
          splProgram: tokenProgramID,
        },
      });
      console.log('Transfer successful');
    } catch (error) {
      console.error('Error transferring SPL tokens:', error);
    }
  };
  
  // Function to transfer Solana lamports
  export const transferSolanaLamports = async (amount: number, to: PublicKey) => {
    try {
      await program.rpc.transferSolanaLamports(amount, to, {
        accounts: {
          account: program.account,
          to,
        },
        lamports: amount,
      });
      console.log('Transfer successful');
    } catch (error) {
      console.error('Error transferring Solana lamports:', error);
    }
  };
  
  // Function to create a private wallet account
  export const createPrivateWalletAccount = async () => {
    try {
      await program.rpc.createPrivateWalletAccount({
        accounts: {
          user: provider.wallet.publicKey,
          wallet: // Provide the private wallet account,
        },
      });
      console.log('Private wallet account created');
    } catch (error) {
      console.error('Error creating private wallet account:', error);
    }
  };
  
  // Function to delete a private wallet account
  export const deletePrivateWalletAccount = async () => {
    try {
      await program.rpc.deletePrivateWalletAccount({
        accounts: {
          wallet: // Provide the private wallet account,
        },
      });
      console.log('Private wallet account deleted');
    } catch (error) {
      console.error('Error deleting private wallet account:', error);
    }
  };
  
  // Function to get the balance of a private wallet account
  export const getPrivateWalletBalance = async () => {
    try {
      const balance = await program.account.privateWallet.fetch(
        // Provide the private wallet account,
      );
      console.log('Private wallet balance:', balance.balance);
      return balance.balance;
    } catch (error) {
      console.error('Error getting private wallet balance:', error);
      return 0;
    }
  };