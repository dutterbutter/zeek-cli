# zeek Gas CLI Tool

The `zeek` CLI tool provides various gas-related functionalities for interacting with the ZKsync network. It allows developers to estimate transaction fees, retrieve gas prices, and access fee parameters, making it easier to plan and optimize transactions on ZKsync.

## Features

- **Estimate Transaction Fees**: Calculate the estimated fee for a transaction on ZKsync.
- **Estimate Gas for L1 to L2 Transactions**: Determine the gas required for transactions that interact with both L1 and L2.
- **Retrieve Gas Prices**: Get the current gas prices on both L1 and L2 networks.
- **Access Fee Parameters**: Fetch the current fee parameters from the ZKsync network.
- **Custom RPC URLs**: Specify custom RPC endpoints to interact with different networks (e.g., mainnet, testnet).

## Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) and Cargo installed on your system. Then, clone the repository and build the tool:

```bash
git clone https://github.com/dutterbutter/zeek-cli.git
cd zeek-cli
cargo build --release
```

The compiled binary will be located at `./target/release/zeek`.

## Usage

The `zeek` CLI tool has several subcommands under the `gas` command. Below are the available commands and their usage.

### Global Options

- `--rpc-url <URL>`: Specify a custom RPC URL (defaults to `https://mainnet.era.zksync.io`).

### Commands

#### 1. Estimate Transaction Fee

Estimate the fee required for a transaction on zkSync L2.

```bash
zeek [--rpc-url <URL>] gas estimate-fee [OPTIONS]
```

**Options:**

- `--from <ADDRESS>`: Sender's address.
- `--to <ADDRESS>`: Recipient's address.
- `--value <AMOUNT>`: Amount to send (in ETH).
- `--gas-limit <GAS_LIMIT>`: Gas limit for the transaction.
- `--gas-price <GAS_PRICE>`: Gas price (in Gwei).
- `--data <DATA>`: Data payload for the transaction (default is `0x`).
- `--show-pubdata`: Display pubdata costs.

**Example:**

```bash
zeek gas estimate-fee \
  --from 0xYourAddress \
  --to 0xRecipientAddress \
  --value 0.001 \
  --gas-limit 21000 \
  --gas-price 100
```

**Output:**

```
Gas Limit: 266444
Max Fee Per Gas: 0.05 Gwei
Max Priority Fee Per Gas: 0.00 Gwei
Gas Per Pubdata Limit: 1549
```

#### 2. Estimate Gas for L1 to L2 Transactions

Estimate the gas required for transactions that interact with both L1 and L2.

```bash
zeek [--rpc-url <URL>] gas estimate-l1-to-l2 [OPTIONS]
```

**Options:**

- `--from <ADDRESS>`: Sender's address.
- `--to <ADDRESS>`: Recipient's address (required).
- `--value <AMOUNT>`: Amount to send (in ETH).
- `--data <DATA>`: Data payload for the transaction (default is `0x`).

**Example:**

```bash
zeek gas estimate-l1-to-l2 \
  --from 0xYourAddress \
  --to 0xRecipientAddress \
  --value 0.001
```

**Output:**

```
Estimated Gas for L1 to L2 Transaction: 3999995
```

#### 3. Get Fee Parameters

Retrieve the current fee parameters from the ZKsync network.

```bash
zeek [--rpc-url <URL>] gas fee-params
```

**Example:**

```bash
zeek gas fee-params
```

**Output:**

```
Fee Parameters:
Minimal L2 Gas Price: 25000000
Compute Overhead Part: 0.0
Pubdata Overhead Part: 1.0
Batch Overhead L1 Gas: 800000
Max Gas Per Batch: 200000000
Max Pubdata Per Batch: 240000
L1 Gas Price: 46226388803
L1 Pubdata Price: 100780475095
```

#### 4. Get Current L1 Gas Price

Retrieve the current gas price on L1.

```bash
zeek [--rpc-url <URL>] gas l1-gas-price
```

**Example:**

```bash
zeek gas l1-gas-price
```

**Output:**

```
Current L1 Gas Price: 46.23 Gwei
```

#### 5. Get Current L2 Gas Price

Retrieve the current gas price on L2.

```bash
zeek [--rpc-url <URL>] gas gas-price
```

**Example:**

```bash
zeek gas gas-price
```

**Output:**

```
Current L2 Gas Price: 0.02 Gwei
```

## Examples

**Estimate Transaction Fee with Custom RPC URL:**

```bash
zeek --rpc-url https://sepolia.era.zksync.dev gas estimate-fee \
  --from 0xYourAddress \
  --to 0xRecipientAddress \
  --value 0.001 \
  --gas-limit 21000 \
  --gas-price 100
```

**Get Current Fee Parameters:**

```bash
zeek gas fee-params
```

**Estimate Gas for L1 to L2 Transaction:**

```bash
zeek gas estimate-l1-to-l2 \
  --from 0xYourAddress \
  --to 0xRecipientAddress \
  --value 0.002 \
  --data 0xabcdef
```

## Notes

- Ensure that the addresses provided are valid Ethereum addresses.
- The amount values for `--value` should be specified in ETH.
- Gas prices should be provided in Gwei.
- Data payloads should be in hexadecimal format, prefixed with `0x`.
- If you don't specify the `--rpc-url`, the tool defaults to `https://mainnet.era.zksync.io`.
- Use the `--show-pubdata` option with `estimate-fee` to display the pubdata cost associated with the transaction.

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue on GitHub.
