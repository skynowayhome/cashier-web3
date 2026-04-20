Here is the English version of the `README.md` for the Cashier (Point of Sale) application. You can copy and paste this directly into your repository:

***

# Stellar Cashier DApp

**Stellar Cashier DApp** - Blockchain-Based Decentralized Point of Sale (POS) System

## Project Description

Stellar Cashier DApp is a decentralized smart contract solution built on the Stellar blockchain using the Soroban SDK. It provides a secure, immutable platform for recording sales transactions directly on the blockchain. This smart contract ensures that your business's sales data is stored transparently and can only be managed through predefined functions, eliminating reliance on centralized databases that are vulnerable to manipulation or data loss.

The system allows cashiers to record new transactions, view sales history, and cancel transactions, leveraging the efficiency and security of the Stellar network. Each transaction is uniquely identified and stored within the contract's instance storage, ensuring the persistence and reliability of your financial data.

## Project Vision

Our vision is to revolutionize financial record-keeping and cashier systems in the digital age by:

- **Decentralizing Sales Data**: Moving transaction records from centralized servers to a global, distributed blockchain.
- **Preventing Fraud & Data Manipulation**: Guaranteeing business owners that sales data cannot be silently altered by unauthorized parties.
- **Guaranteeing Immutability**: Providing a permanent, tamper-proof record of every transaction.
- **Building Trustless Systems**: Creating a platform where financial data integrity is guaranteed by code, rather than manual reporting.

We envision a future where Point of Sale (POS) systems can operate autonomously, transparently, and be audited in real-time by business owners anywhere in the world.

## Key Features

### 1. **Automated Transaction Recording**
- Record sales with just one function call.
- Input the item name, quantity, and price per item; the smart contract calculates the total price automatically.
- Auto-generated unique IDs for every receipt/transaction.

### 2. **Efficient Data Retrieval**
- Fetch the entire stored transaction history in a single call.
- Structured data representation for seamless frontend integration (e.g., admin or cashier dashboards).
- Real-time synchronization with the blockchain state.

### 3. **Secure Deletion & Cancellation**
- Remove or cancel specific transactions using their unique IDs (useful for refunds or input errors).
- Immediate update of the transaction list following a cancellation.

### 4. **Transparency and Auditability**
- View all cashier activities publicly on the blockchain ledger.
- Blockchain-based verification of all financial entries.
- Immutable audit trails perfectly suited for bookkeeping and financial audits.

### 5. **Stellar Network Integration**
- Leverages the high speed and ultra-low transaction costs of the Stellar network.
- Built using the modern Soroban Smart Contract SDK.
- Scalable architecture designed to handle growing store transaction volumes.

## Contract Details

- **Contract Address :** `CAKO5H2EN2OQD4QTRW43QYPBTW2ZUKVQ7R5SC5D6UIJWSMV6SNS2UN4W`

## Future Scope

### Short-Term Enhancements
1. **Inventory Management**: Add functionality to track and automatically deduct stock levels when a transaction occurs.
2. **Product Catalog**: Store product names and prices within the contract state so cashiers only need to input a product ID.
3. **Discounts & Taxes**: Support for calculating discount percentages and tax (VAT) before the final price is stored.

### Medium-Term Development
4. **On-Chain Settlement**: Integrate tokens or Stablecoins (like USDC on the Stellar network) so customers can pay directly via their crypto wallets.
5. **Role-Based Access Control**: Allow multiple addresses (cashiers) to input data, while restricting data deletion/cancellation exclusively to the store owner (Admin).
6. **Digital Receipts**: Build an off-chain bridge to generate and send digital receipts to customers.

### Long-Term Vision
7. **Web3 Loyalty Programs**: Automate the distribution of reward tokens or NFT-based discounts to returning customers upon transaction completion.
8. **AI-Powered Sales Prediction**: Analyze on-chain sales data using off-chain AI to predict trends and best-selling items.
9. **Decentralized UI Hosting**: Host the cashier frontend on IPFS or similar decentralized platforms.

### Enterprise Features
10. **Multi-Branch Synchronization**: Adapt the system for large retail chains with cross-branch transaction tracking and identification.
11. **Automated Tax Reporting**: Set automated triggers that summarize monthly revenue to streamline tax reporting.
12. **Supply Chain Integration**: Connect sales data with supplier smart contracts for automated inventory restocking.

---

## Technical Requirements

- Soroban SDK
- Rust programming language
- Stellar blockchain network (Futurenet / Testnet / Mainnet)

## Getting Started

Deploy this smart contract to Stellar's Soroban network and interact with it using the three main functions:

- `add_transaction(item_name, quantity, price_per_item)` - Record a new transaction. The total price will be calculated automatically within the contract.
- `get_transactions()` - Retrieve the list of all recorded sales transactions.
- `delete_transaction(id)` - Remove or cancel a specific transaction by its unique ID.

---

**Stellar Cashier DApp** - Securing and Modernizing Your Business Bookkeeping on the Blockchain