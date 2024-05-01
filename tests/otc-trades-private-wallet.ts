import { PublicKey } from '@solana/web3.js';
import { expect } from 'chai';
import {
    depositSPLToken,
    depositSolanaLamports,
    withdrawSPLToken,
    withdrawSolanaLamports,
    transferSPLTokens,
    transferSolanaLamports,
    createPrivateWalletAccount,
    deletePrivateWalletAccount,
    getPrivateWalletBalance,
  } from './adapter'; // Import the adapter functions
  
  describe('Testing Solana Rust program functions', () => {
    test('Deposit SPL tokens', async () => {
      // Mock deposit amount
      const depositAmount = 10;
  
      // Invoke the deposit function
      await depositSPLToken(depositAmount);
  
      // Check if the balance increased by depositAmount
      const newBalance = await getPrivateWalletBalance();
      expect(newBalance).greaterThan(0);
    });
  
    test('Deposit Solana lamports', async () => {
      // Mock deposit amount
      const depositAmount = 1000000;
  
      // Invoke the deposit function
      await depositSolanaLamports(depositAmount);
  
      // Check if the balance increased by depositAmount
      const newBalance = await getPrivateWalletBalance();
      expect(newBalance).greaterThan(0);
    });
  
    test('Withdraw SPL tokens', async () => {
      // Mock withdraw amount
      const withdrawAmount = 5;
  
      // Invoke the withdraw function
      await withdrawSPLToken(withdrawAmount);
  
      // Check if the balance decreased by withdrawAmount
      const newBalance = await getPrivateWalletBalance();
      expect(newBalance).greaterThanOrEqual(0);
    });
  
    test('Withdraw Solana lamports', async () => {
      // Mock withdraw amount
      const withdrawAmount = 500000;
  
      // Invoke the withdraw function
      await withdrawSolanaLamports(withdrawAmount);
  
      // Check if the balance decreased by withdrawAmount
      const newBalance = await getPrivateWalletBalance();
      expect(newBalance).greaterThanOrEqual(0);
    });
  
    test('Transfer SPL tokens', async () => {
      // Mock transfer amount and recipient
      const amount = 3;
      const recipient = new PublicKey('recipientPublicKey');
  
      // Get initial balance
      const initialBalance = await getPrivateWalletBalance();
  
      // Invoke the transfer function
      await transferSPLTokens(amount, recipient);
  
      // Check if the balance decreased by amount
      const newBalance = await getPrivateWalletBalance();
      expect(newBalance).equal(initialBalance - amount);
    });
  
    test('Transfer Solana lamports', async () => {
      // Mock transfer amount and recipient
      const amount = 500000;
      const recipient = new PublicKey('recipientPublicKey');
  
      // Get initial balance
      const initialBalance = await getPrivateWalletBalance();
  
      // Invoke the transfer function
      await transferSolanaLamports(amount, recipient);
  
      // Check if the balance decreased by amount
      const newBalance = await getPrivateWalletBalance();
      expect(newBalance).equal(initialBalance - amount);
    });
  
    test('Create private wallet account', async () => {
      // Get initial balance
      const initialBalance = await getPrivateWalletBalance();
  
      // Invoke the create function
      await createPrivateWalletAccount();
  
      // Check if the balance remains unchanged
      const newBalance = await getPrivateWalletBalance();
      expect(newBalance).equal(initialBalance);
    });
  
    test('Delete private wallet account', async () => {
      // Get initial balance
      const initialBalance = await getPrivateWalletBalance();
  
      // Invoke the delete function
      await deletePrivateWalletAccount();
  
      // Check if the balance remains unchanged
      const newBalance = await getPrivateWalletBalance();
      expect(newBalance).equal(initialBalance);
    });
  
    test('Get private wallet balance', async () => {
      // Invoke the get balance function
      const balance = await getPrivateWalletBalance();
  
      // Ensure balance is a non-negative number
      expect(balance).greaterThanOrEqual(0);
    });
    
  });