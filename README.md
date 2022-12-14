# Uniswap V2 Router - Casper Blockchain

Router uses other contracts (factory, pair, library, etc) and add, remove, swap tokens from liquidity pools.

## Security Review Status

![QuantstampSecured](https://s3-us-west-1.amazonaws.com/qsp-www-images/certificate-gh-badge.svg)

[View Report - commit 237cf3fa15ee95843603ab3857789cf2b53d58b7](RengoLabs-Report.pdf)

## Contents

There are 2 contracts in this repo

1. uniswap-v2-library
2. uniswap-v2-router

## Table of contents

- [Interacting with the contract](#interacting-with-the-contract)
  - [Install the prerequisites](#install-the-prerequisites)
  - [All Test Cases](#all-test-cases)
  - [Known contract hashes](#known-contract-hashes)
  - [Deploying Router contract manually](#deploying-router-contract-manually)
    - [Manual Deployment](#manual-deployment)
      - [Factory](#factory)
      - [Wcspr](#wcspr)
      - [Library](#library)
      - [Pair](#pair)
    - [Entry Point methods](#entry-point-methods)
      - [add_liquidity](#add_liquidity)
      - [add_liquidity_cspr](#add_liquidity_cspr)
      - [remove_liquidity](#remove_liquidity)
      - [remove_liquidity_cspr](#remove_liquidity_cspr)
      - [swap_exact_tokens_for_tokens](#swap_exact_tokens_for_tokens)
      - [swap_tokens_for_exact_tokens](#swap_tokens_for_exact_tokens)
      - [swap_exact_cspr_for_tokens](#swap_exact_cspr_for_tokens)
      - [swap_tokens_for_exact_cspr](#swap_tokens_for_exact_cspr)
      - [swap_exact_tokens_for_cspr](#swap_exact_tokens_for_cspr)
      - [swap_cspr_for_exact_tokens](#swap_cspr_for_exact_tokens)
      - [quote](#quote)
      - [get_amount_out](#get_amount_out)
      - [get_amount_in](#get_amount_in)
      - [get_amounts_out](#get_amounts_out)
      - [get_amounts_in](#get_amounts_in)
      - [receive](#receive)
      - [change_owner](#change_owner)
      - [add_to_whitelist](#add_to_whitelist)
      - [remove_from_whitelist](#remove_from_whitelist)
  - [Deploying Library contract manually](#deploying-library-contract-manually)
    - [Entry Point methods](#library-entry-point-methods)
      - [sort_tokens](#library_sort_tokens)
      - [get_reserves](#library_get_reserves)
      - [quote](#library_quote)
      - [get_amount_out](#library_get_amount_out)
      - [get_amount_in](#library_get_amount_in)
      - [get_amounts_out](#library_get_amounts_out)
      - [get_amounts_in](#library_get_amounts_in)
      - [pair_for](#library_pair_for)

## Interacting with the contract

You need to have `casper-client` and `jq` installed on your system to run the examples. The instructions have been tested on Ubuntu 20.04.0 LTS.

## Install the prerequisites

You can install the required software by issuing the following commands. If you are on an up-to-date Casper node, you probably already have all of the prerequisites installed so you can skip this step.

## Note: If any command fails try again by restarting the terminal to reset the enviornment variable.

## Update package repositories

```
sudo apt update
```

## Install the command-line JSON processor

```
sudo apt install jq -y
```

## Install rust

Choose cutomize intallation to install nightly version
Install the nightly version (by default stable toolchain is installed)

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```
rustup install nightly-2022-08-29
```

## Check that nightly toolchain version is installed(this will list stable and nightly versions)

```
rustup toolchain list
```

## Set rust nightly as default

```
rustup default nightly-2022-08-29-x86_64-unknown-linux-gnu
```

## Install wasm32-unknown-unknown

```
rustup target add wasm32-unknown-unknown
```

## Rust Version

```
rustup --version
```

## Install Cmake

```
sudo apt-get -y install cmake
```

Note:https://cgold.readthedocs.io/en/latest/first-step/installation.html

## Check if cmake is installed properly

```
cmake --version
```

## Install the Casper Crates

```
cargo install cargo-casper
```

## Add Casper repository

```
echo "deb https://repo.casperlabs.io/releases" bionic main | sudo tee -a /etc/apt/sources.list.d/casper.list
```

```
curl -O https://repo.casperlabs.io/casper-repo-pubkey.asc
```

```
sudo apt-key add casper-repo-pubkey.asc
```

```
sudo apt update
```

```
sudo apt install libssl-dev
```

```
sudo apt install pkg-config
```

## Install the Casper client software

```
cargo +nightly-2022-08-29-x86_64-unknown-linux-gnu install casper-client
```

## To check Casper Client Version

```
casper-client --version
```

# Additonal commands for help

```
casper-client --help
casper-client <command> --help
```

## Generate the keys

```
casper-client keygen keys
```

## Fund the key

The keys can be funded from casper live website [testnet faucet](https://testnet.cspr.live/tools/faucet). Requires chrome browser and the casper signer extension. You should import the keys that were generated in the previous step

## Install

Make sure `wasm32-unknown-unknown` is installed.

```
  make prepare
```

It's also recommended to have [wasm-strip](https://github.com/WebAssembly/wabt)
available in your PATH to reduce the size of compiled Wasm.

### Build Dependencies

Run this command to build specific smart contract.

```
  make build-dependencies
```

### Build Smart Contract

Run this command to build specific smart contract.

```
  make build-contract-uniswap-v2-library
  make build-contract-uniswap-v2-router
```

### Build All Smart Contracts

Run this command in main folder to build all Smart Contract.

```
  make build-all
```

### All Test Cases<a name="all-test-cases"></a>

Tests require that the CasperLabs-UniswapV2-core repository to be checked out
into a sibling directory (one up from the current directory).

```
cd ..

git clone git@github.com:Rengo-Labs/uniswap-v2-core-casper.git
```

To build the contracts and run the tests, first navigate back to this directory
and run:

```
make all
```

To run all the tests:

```
make test-all
```

### Clean Command

To Clean up

```
make clean
```

### Known contract hashes <a name="known-contract-hashes"></a>

Router contract has already being deployed. Inorder to interact with it you need to call it by its hash. The table below contains the contract hash (without the `hash-` prefix) for Router contract on public Casper networks:

| Network | Account info contract hash                                              | Contract owner     |
| ------- | ----------------------------------------------------------------------- | ------------------ |
| Testnet | `hash-d52d2e98554c1854fd8a9ce541a9d52dab73fd2841655513a9c8295898803ce0` | Casper Association |

### Deploying Router contract manually <a name="deploying-router-contract-manually"></a>

If you need to deploy the router contract manually you need to pass the hashes of the other contracts as parameter. Following is the command to deploy the Router contract.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 250000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="factory:Key='Hash of factory Contract'" \
    --session-arg="wcspr:Key='Hash of WCSPR Contract'" \
    --session-arg="library:Key='Hash of Library Contract'" \
    --session-arg="contract_name:string='contract_name'"
```

Before deploying Router Contract, you would need to deploy other contracts first and pass hashes of these contracts to the respective parameters above. We have already deployed these contracts and the tables belows displays the hashes of the contracts.

| Name    | Network | Account info contract hash                                              | Contract owner     |
| ------- | ------- | ----------------------------------------------------------------------- | ------------------ |
| Factory | Testnet | `hash-5028190b8a5b6addbf3d51ee2c6ae5b913f09223d65eff9bcf5985f74ae976ec` | Casper Association |
| Wcspr   | Testnet | `hash-083756dee38a7e3a8a7190a17623cfbc8bc107511de206f03c3dbd1af5463a45` | Casper Association |
| Library | Testnet | `hash-fa073d1a95a606871983689633dab9464fb5fbe5f723b0855e025ea01b9bf308` | Casper Association |
| Pair    | Testnet | `hash-8e6fbaae9f5ff3bb3cca7cb15723b2a47917d074922575187cb136e8d4b169a7` | Casper Association |

### Manual Deployment <a name="manual-deployment"></a>

For manual deployments of these contracts, following are the commands.

#### Factory <a name="factory"></a>

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 150000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="fee_to_setter:Key='Hash of fee-to-setter Contract'" \
    --session-arg="contract_name:string='contract_name'"
```

#### Wcspr <a name="wcspr"></a>

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 345000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:string='token-name'" \
    --session-arg="symbol:string='token-symbol'" \
    --session-arg="decimals:u8='unsigned integer value'" \
    --session-arg="initial_supply:u256='unsigned integer value'" \
    --session-arg="contract_name:string='contract_name'"
```

#### Library <a name="library"></a>

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 120000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="contract_name:string='contract_name'"
```

#### Pair <a name="pair"></a>

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 440000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:string='token-name'" \
    --session-arg="symbol:string='token-symbol'" \
    --session-arg="decimals:u8='unsigned integer value'" \
    --session-arg="initial_supply:u256='unsigned integer value'" \
    --session-arg="factory_hash:Key='Hash of factory Contract'" \
    --session-arg="callee_contract_hash:Key='Callee Contract Hash'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a name="entry-point-methods"></a>

Following are the Router's entry point methods.

- ### add_liquidity <a name="add_liquidity"></a>

  This method adds liquidity to ERC-20⇄ERC-20 pool.
  <br>To cover all possible scenarios, msg.sender should have already given the router an allowance of at least amount_a_desired/amount_b_desired on token_a/token_b.
  <br>Always adds assets at the ideal ratio, according to the price when the transaction is executed.

  Following is the table of parameters.

  | Parameter Name   | Type          |
  | ---------------- | ------------- |
  | token_a          | Key           |
  | token_b          | Key           |
  | amount_a_desired | U256          |
  | amount_b_desired | U256          |
  | amount_a_min     | U256          |
  | amount_b_min     | U256          |
  | to               | KEY           |
  | deadline         | U256          |
  | pair             | Option`<Key>` |

  This method **returns** `Tuple3(U256,U256,U256)`

- ### add_liquidity_cspr <a name="add_liquidity_cspr"></a>

  This method adds liquidity to ERC-20⇄CSPR pool with CSPR.
  <br>To cover all possible scenarios, msg.sender should have already given the router an allowance of at least amount_token_desired on token.
  <br>Always adds assets at the ideal ratio, according to the price when the transaction is executed.
  <br>Left over cspr if any is returned to msg.sender

  Following is the table of parameters.

  | Parameter Name       | Type          |
  | -------------------- | ------------- |
  | token                | Key           |
  | amount_token_desired | U256          |
  | amount_cspr_desired  | U256          |
  | amount_token_min     | U256          |
  | amount_cspr_min      | U256          |
  | to                   | KEY           |
  | deadline             | U256          |
  | pair                 | Option`<Key>` |
  | purse                | URef          |

  This method **returns** `Tuple3(U256,U256,U256)`

- ### remove_liquidity <a name="remove_liquidity"></a>

  This method Removes liquidity from an ERC-20⇄ERC-20 pool.
  <br>msg.sender should have already given the router an allowance of at least liquidity on the pool.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | token_a        | Key  |
  | token_b        | Key  |
  | liquidity      | U256 |
  | amount_a_min   | U256 |
  | amount_b_min   | U256 |
  | to             | Key  |
  | deadline       | U256 |

  This method **returns** `Tuple2(U256,U256)`

- ### remove_liquidity_cspr <a name="remove_liquidity_cspr"></a>

  This method Removes liquidity from an ERC-20⇄ERC-20 pool.
  <br>msg.sender should have already given the router an allowance of at least liquidity on the pool.

  Following is the table of parameters.

  | Parameter Name   | Type |
  | ---------------- | ---- |
  | token            | Key  |
  | liquidity        | U256 |
  | amount_token_min | U256 |
  | amount_cspr_min  | U256 |
  | to               | Key  |
  | deadline         | U256 |
  | to_purse         | URef |

  This method **returns** `Tuple2(U256,U256)`

- ### swap_exact_tokens_for_tokens <a name="swap_exact_tokens_for_tokens"></a>

  Swaps an exact amount of input tokens for as many output tokens as possible, along the route determined by the path. The first element of path is the input token, the last is the output token, and any intermediate elements represent intermediate pairs to trade through (if, for example, a direct pair does not exist).
  <br>msg.sender should have already given the router an allowance of at least amount_in on the input token.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | amount_in      | U256          |
  | amount_out_min | U256          |
  | path           | Vec`<String>` |
  | to             | Key           |
  | deadline       | U256          |

  This method **returns** `Vec<U256>`

- ### swap_tokens_for_exact_tokens <a name="swap_tokens_for_exact_tokens"></a>

  Receive an exact amount of output tokens for as few input tokens as possible, along the route determined by the path. The first element of path is the input token, the last is the output token, and any intermediate elements represent intermediate tokens to trade through (if, for example, a direct pair does not exist).
  <br>msg.sender should have already given the router an allowance of at least amount_in_max on the input token.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | amount_out     | U256          |
  | amount_in_max  | U256          |
  | path           | Vec`<String>` |
  | to             | Key           |
  | deadline       | U256          |

  This method **returns** `Vec<String>`

- ### swap_exact_cspr_for_tokens <a name="swap_exact_cspr_for_tokens"></a>

  Swaps an exact amount of cspr for as many output tokens as possible, along the route determined by the path. The first element of path must be WCSPR, the last is the output token, and any intermediate elements represent intermediate pairs to trade through (if, for example, a direct pair does not exist).

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | amount_out_min | U256          |
  | amount_in      | U256          |
  | path           | Vec`<String>` |
  | to             | Key           |
  | deadline       | U256          |
  | purse          | URef          |

  This method **returns** `Vec<U256>`

- ### swap_tokens_for_exact_cspr <a name="swap_tokens_for_exact_cspr"></a>

  Receive an exact amount of CSPR for as few input tokens as possible, along the route determined by the path. The first element of path is the input token, the last must be WCSPR, and any intermediate elements represent intermediate pairs to trade through (if, for example, a direct pair does not exist).
  <br>msg.sender should have already given the router an allowance of at least amount_in_max on the input token.
  <br>If the to address is a smart contract, it must have the ability to receive cspr.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | amount_out     | U256          |
  | amount_in_max  | U256          |
  | path           | Vec`<String>` |
  | to             | URef          |
  | deadline       | U256          |

  This method **returns** `Vec<U256>`

- ### swap_exact_tokens_for_cspr <a name="swap_exact_tokens_for_cspr"></a>

  Swaps an exact amount of tokens for as much cspr as possible, along the route determined by the path. The first element of path is the input token, the last must be WCSPR, and any intermediate elements represent intermediate pairs to trade through (if, for example, a direct pair does not exist).
  <br>If the to address is a smart contract, it must have the ability to receive cspr.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | amount_in      | U256          |
  | amount_out_min | U256          |
  | path           | Vec`<String>` |
  | to             | URef          |
  | deadline       | U256          |

  This method **returns** `Vec<U256>`

- ### swap_cspr_for_exact_tokens <a name="swap_cspr_for_exact_tokens"></a>

  Receive an exact amount of tokens for as little CSPR as possible, along the route determined by the path. The first element of path must be WCSPR, the last is the output token and any intermediate elements represent intermediate pairs to trade through (if, for example, a direct pair does not exist).
  <br>Leftover CSPR, if any, is returned to msg.sender.

  Following is the table of parameters.

  | Parameter Name | Type          |
  | -------------- | ------------- |
  | amount_out     | U256          |
  | amount_in_max  | U256          |
  | path           | Vec`<String>` |
  | to             | Key           |
  | deadline       | U256          |
  | purse          | URef          |

  This method **returns** `Vec<U256>`

- ### quote <a name="quote"></a>

  Given some amount of an asset and pair reserves, returns an equivalent amount of the other asset.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | amount_a       | U256 |
  | reserve_a      | U256 |
  | reserve_b      | U256 |

  This method **returns** `U256`

- ### get_amount_out <a name="get_amount_out"></a>

  Given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | amount_in      | U256 |
  | reserve_in     | U256 |
  | reserve_out    | U256 |

  This method **returns** `U256`

- ### get_amount_in <a name="get_amount_in"></a>

  Given an output amount of an asset and pair reserves, returns a required input amount of the other asset.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | amount_out     | U256 |
  | reserve_in     | U256 |
  | reserve_out    | U256 |

  This method **returns** `U256`

- ### get_amounts_out <a name="get_amounts_out"></a>

  Performs chained getAmountOut calculations on any number of pairs.

  Following is the table of parameters.

  | Parameter Name | Type       |
  | -------------- | ---------- |
  | amount_in      | U256       |
  | path           | Vec`<Key>` |

  This method **returns** `Vec<U256>`

- ### get_amounts_in <a name="get_amounts_in"></a>

  Performs chained getAmountIn calculations on any number of pairs.

  Following is the table of parameters.

  | Parameter Name | Type       |
  | -------------- | ---------- |
  | amount_out     | U256       |
  | path           | Vec`<Key>` |

  This method **returns** `Vec<U256>`

- ### receive <a name="receive"></a>

  Only accept CSPR via from the WCSPR contract.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | amount         | U512 |
  | purse          | URef |

  This method **returns** nothing.

- ### change_owner <a name="change_owner"></a>

  Change the owner for whitelisting.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | owner          | Key  |

  This method **returns** nothing.

- ### add_to_whitelist <a name="add_to_whitelist"></a>

  Add a user to whitelist.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | user           | Key  |

  This method **returns** nothing.

- ### remove_from_whitelist <a name="remove_from_whitelist"></a>

  Remove a user from whitelist.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | user           | Key  |

  This method **returns** nothing.

### Deploying Library contract manually <a name="deploying-library-contract-manually"></a>

If you need to deploy the `Library contract` manually you need to pass the some parameters. Following is the command to deploy the `Library contract`.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 120000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="contract_name:string='contract_name'"
```

## Entry Point methods <a id="library-entry-point-methods"></a>

Following are the WCSPR's entry point methods.

- ### sort_tokens <a name="library_sort_tokens"></a>

  Return the tokens Contract Package Hash.

  Following is the table of parameters.

  | Parameter Name | Type       |
  | -------------- | ---------- |
  | token_a        | Key        |
  | token_b        | Key        |

  This method **returns** `Tuple2(ContractPackageHash,ContractPackageHash)`

- ### get_reserves <a name="library_get_reserves"></a>

  Return the reserves.

  Following is the table of parameters.

  | Parameter Name | Type       |
  | -------------- | ---------- |
  | factory        | Key        |
  | token_a        | Key        |
  | token_b        | Key        |

  This method **returns** `Tuple2(U128,U128)`

- ### quote <a name="library_quote"></a>

  Given some amount of an asset and pair reserves, returns an equivalent amount of the other asset.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | amount_a       | U256 |
  | reserve_a      | U256 |
  | reserve_b      | U256 |

  This method **returns** `U256`

- ### get_amount_out <a name="library_get_amount_out"></a>

  Given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | amount_in      | U256 |
  | reserve_in     | U256 |
  | reserve_out    | U256 |

  This method **returns** `U256`

- ### get_amount_in <a name="library_get_amount_in"></a>

  Given an output amount of an asset and pair reserves, returns a required input amount of the other asset.

  Following is the table of parameters.

  | Parameter Name | Type |
  | -------------- | ---- |
  | amount_out     | U256 |
  | reserve_in     | U256 |
  | reserve_out    | U256 |

  This method **returns** `U256`

- ### get_amounts_out <a name="library_get_amounts_out"></a>

  Performs chained getAmountOut calculations on any number of pairs.

  Following is the table of parameters.

  | Parameter Name | Type       |
  | -------------- | ---------- |
  | factory        | Key        |
  | amount_in      | U256       |
  | path           | Vec`<Key>` |

  This method **returns** `Vec<U256>`

- ### get_amounts_in <a name="library_get_amounts_in"></a>

  Performs chained getAmountIn calculations on any number of pairs.

  Following is the table of parameters.

  | Parameter Name | Type       |
  | -------------- | ---------- |
  | factory        | Key        |
  | amount_out     | U256       |
  | path           | Vec`<Key>` |

  This method **returns** `Vec<U256>`

- ### pair_for <a name="library_pair_for"></a>

  Returns the pair on the following addresses.

  Following is the table of parameters.

  | Parameter Name | Type       |
  | -------------- | ---------- |
  | factory        | Key        |
  | token_a        | Key        |
  | token_b        | Key        |

  This method **returns** `Key`
