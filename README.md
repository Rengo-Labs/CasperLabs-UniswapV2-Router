# Scytalelabs-UniswapRouter
Router uses other contracts (factory, pair, library, etc) and add, remove, swap tokens from liquidity pools.

## Table of contents

- [Interacting with the contract](#interacting-with-the-contract)
  - [Install the prerequisites](#install-the-prerequisites)
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
    - [remove_liquidity_with_permit](#remove_liquidity_with_permit)
    - [remove_liquidity_cspr_with_permit](#remove_liquidity_cspr_with_permit)
    - [swap_exact_tokens_for_tokens](#swap_exact_tokens_for_tokens)
    - [swap_tokens_for_exact_tokens](#swap_tokens_for_exact_tokens)
    - [swap_exact_cspr_for_tokens](#swap_exact_cspr_for_tokens)
    - [swap_tokens_for_exact_cspr](#swap_tokens_for_exact_cspr)
    - [swap_exact_tokens_for_cspr](#swap_exact_tokens_for_cspr)
    - [swap_cspr_for_exact_tokens](#swap_cspr_for_exact_tokens)

## Interacting with the contract <a name="interacting-with-the-contract"></a>
You need to have ```casper-client``` and ```jq``` installed on your system to run the examples. The instructions have been tested on Ubuntu 20.04.2 LTS.

### Install the prerequisites <a name="install-the-prerequisites"></a>

You can install the required software by issuing the following commands. If you are on an up-to-date Casper node, you probably already have all of the prerequisites installed so you can skip this step.

```bash
# Update package repositories
sudo apt update

# Install the command-line JSON processor
sudo apt install jq -y

# Add Casper repository
echo "deb https://repo.casperlabs.io/releases" bionic main | sudo tee -a /etc/apt/sources.list.d/casper.list
curl -O https://repo.casperlabs.io/casper-repo-pubkey.asc
sudo apt-key add casper-repo-pubkey.asc
sudo apt update

# Install the Casper client software
sudo apt install casper-client -y
```

### Known contract hashes <a name="known-contract-hashes"></a>

Router contract has already being deployed. Inorder to interact with it you need to call it by its hash. The table below contains the contract hash (without the ```hash-``` prefix) for Router contract on public Casper networks:

Network | Account info contract hash | Contract owner
---|---|---
Testnet | ```hash-d52d2e98554c1854fd8a9ce541a9d52dab73fd2841655513a9c8295898803ce0``` | Casper Association


### Deploying Router contract manually <a name="deploying-router-contract-manually"></a>

If you need to deploy the router contract manually you need to pass the hashes of the other contracts as parameter. Following is the command to deploy the Router contract.

```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="factory:Key='Hash of factory Contract'" \
    --session-arg="wcspr:Key='Hash of WCSPR Contract'" \
    --session-arg="library:Key='Hash of Library Contract'" \
    --session-arg="pair:Key='Hash of Pair Contract'" \
    --session-arg="purse:opt_uref='if the caller is purse pass its contract, if the caller is account pass Null/None'" \
    --session-arg="contract_name:string='contract_name'"
```


Before deploying Router Contract, you would need to deploy other contracts first and pass hashes of these contracts to the respective parameters above. We have already deployed these contracts and the tables belows displays the hashes of the contracts.

Name | Network | Account info contract hash | Contract owner
---|---|---|---
Factory | Testnet | ```hash-7272481d5b5c8d1a245708f5ca40a07d93bd180ceeb9c2e0dd6b2f295e6328b2``` | Casper Association
Wcspr | Testnet | ```hash-b707db44a84944dd6844f7582f53846211effa3663a5876396848f31d2cf5976``` | Casper Association
Library | Testnet | ```hash-e3dae47f3c42dec089c880d35f2a56ee03b7623bbd15c2abc670659cd497ce85``` | Casper Association
Pair | Testnet | ```hash-8627541e52220fba484c39fd7a8acea38c15082f710a522b815354aa46d9451b``` | Casper Association


### Manual Deployment <a name="manual-deployment"></a>

For manual deployments of these contracts, following are the commands.

#### Factory <a name="factory"></a>
```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
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
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:string='token-name'" \
    --session-arg="symbol:string='token-symbol'" \
    --session-arg="contract_name:string='contract_name'"
```

#### Library <a name="library"></a>
```bash
sudo casper-client put-deploy \
    --chain-name chain_name \
    --node-address http://$NODE_ADDRESS:7777/ \
    --secret-key path_to_secret_key.pem \
    --session-path path_to_wasm_file \
    --payment-amount 10000000000 \
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
    --payment-amount 10000000000 \
    --session-arg="public_key:public_key='Public Key In Hex'" \
    --session-arg="name:string='token-name'" \
    --session-arg="symbol:string='token-symbol'" \
    --session-arg="decimals:u8='unsigned integer value'" \
    --session-arg="initial_supply:u256='unsigned integer value'" \
    --session-arg="contract_name:string='contract_name'"
    --session-arg="factory_hash:Key='Hash of factory Contract'" \
    --session-arg="callee_contract_hash:Key='Callee Contract Hash'" \
```

## Entry Point methods <a name="entry-point-methods"></a>

Following are the Router's entry point methods.

- ### add_liquidity <a name="add_liquidity"></a>
This method adds liquidity to ERC-20⇄ERC-20 pool.
<br>To cover all possible scenarios, msg.sender should have already given the router an allowance of at least amount_a_desired/amount_b_desired on token_a/token_b.
<br>Always adds assets at the ideal ratio, according to the price when the transaction is executed.

Following is the table of parameters.

Parameter Name | Type
---|---
token_a | Key 
token_b | Key
amount_a_desired | U256
amount_b_desired | U256
amount_a_min | U256
amount_b_min | U256
to | KEY
deadline (epoch in milliseconds) | U256

This method **returns** ```amount_a:U256, amount_b:U256, liquidity:U256```


- ### add_liquidity_cspr <a name="add_liquidity_cspr"></a>
This method adds liquidity to ERC-20⇄CSPR pool with CSPR. 
<br>To cover all possible scenarios, msg.sender should have already given the router an allowance of at least amount_token_desired on token.
<br>Always adds assets at the ideal ratio, according to the price when the transaction is executed.
<br>Left over cspr if any is returned to msg.sender


Following is the table of parameters.

Parameter Name | Type
---|---
token | Key 
amount_token_desired | U256
amount_cspr_desired | U256
amount_token_min | U256
amount_cspr_min | U256
to | KEY
deadline (epoch in milliseconds) | U256
purse | CLType::Option(Box::new(CLType::URef))

This method **returns** ```amount_token:U256, amount_cspr:U256, liquidity:U256```



- ### remove_liquidity <a name="remove_liquidity"></a>
This method Removes liquidity from an ERC-20⇄ERC-20 pool. 
<br>msg.sender should have already given the router an allowance of at least liquidity on the pool.


Following is the table of parameters.

Parameter Name | Type
---|---
token_a | Key 
token_b | Key
liquidity | U256
amount_a_min | U256
amount_b_min | U256
to | Key
deadline (epoch in milliseconds) | U256

This method **returns** ```amount_a:U256, amount_b:U256```


- ### remove_liquidity_cspr <a name="remove_liquidity_cspr"></a>
This method Removes liquidity from an ERC-20⇄ERC-20 pool. 
<br>msg.sender should have already given the router an allowance of at least liquidity on the pool.


Following is the table of parameters.

Parameter Name | Type
---|---
token | Key 
liquidity | U256
amount_token_min | U256
amount_cspr_min | U256
to | Key
deadline (epoch in milliseconds) | U256

This method **returns** ```amount_token:U256, amount_cspr:U256```


- ### remove_liquidity_with_permit <a name="remove_liquidity_with_permit"></a>
This method Removes liquidity from an ERC-20⇄ERC-20 pool without pre-approval.


Following is the table of parameters.

Parameter Name | Type
---|---
token_a | Key 
token_b | Key
liquidity | U256
amount_a_min | U256
amount_b_min | U256
to | Key
deadline (epoch in milliseconds) | U256
approve_max | Bool
public_key | String
signature | String


This method **returns** ```amount_a:U256, amount_b:U256```
<br>**Note:** To know the steps of calculating the signature, refer to the documentation of Pair contract, in the pair repository.


- ### remove_liquidity_cspr_with_permit <a name="remove_liquidity_cspr_with_permit"></a>
This method Removes liquidity from an ERC-20⇄ERC-20 pool without pre-approval.


Following is the table of parameters.

Parameter Name | Type
---|---
token | Key 
liquidity | U256
amount_token_min | U256
amount_cspr_min | U256
to | Key
deadline (epoch in milliseconds) | U256
approve_max | Bool
public_key | String
signature | String

This method **returns** ```amount_token:U256, amount_cspr:U256```
<br>**Note:** To know the steps of calculating the signature, refer to the documentation of Pair contract, in the pair repository.


- ### swap_exact_tokens_for_tokens <a name="swap_exact_tokens_for_tokens"></a>
Swaps an exact amount of input tokens for as many output tokens as possible, along the route determined by the path. The first element of path is the input token, the last is the output token, and any intermediate elements represent intermediate pairs to trade through (if, for example, a direct pair does not exist).
<br>msg.sender should have already given the router an allowance of at least amount_in on the input token.

Following is the table of parameters.

Parameter Name | Type
---|---
amount_in | U256
amount_out_min | U256
path | Vec<Key>
to | Key
deadline (epoch in milliseconds) | U256

This method **returns** ```amounts: Vector<U256>```

    
- ### swap_tokens_for_exact_tokens <a name="swap_tokens_for_exact_tokens"></a>
Receive an exact amount of output tokens for as few input tokens as possible, along the route determined by the path. The first element of path is the input token, the last is the output token, and any intermediate elements represent intermediate tokens to trade through (if, for example, a direct pair does not exist).
<br>msg.sender should have already given the router an allowance of at least amount_in_max on the input token.

Following is the table of parameters.

Parameter Name | Type
---|---
amount_out | U256
amount_in_max | U256
path | Vec<Key>
to | Key
deadline (epoch in milliseconds) | U256

This method **returns** ```amounts: Vector<U256>```
    

- ### swap_exact_cspr_for_tokens <a name="swap_exact_cspr_for_tokens"></a>
Swaps an exact amount of cspr for as many output tokens as possible, along the route determined by the path. The first element of path must be WCSPR, the last is the output token, and any intermediate elements represent intermediate pairs to trade through (if, for example, a direct pair does not exist).

Following is the table of parameters.

Parameter Name | Type
---|---
amount_out_min | U256
amount_in | U256
path | Vec<Key>
to | Key
deadline (epoch in milliseconds) | U256
purse | CLType::Option(Box::new(CLType::URef))

This method **returns** ```amounts: Vector<U256>```
    
    
- ### swap_tokens_for_exact_cspr <a name="swap_tokens_for_exact_cspr"></a>
Receive an exact amount of CSPR for as few input tokens as possible, along the route determined by the path. The first element of path is the input token, the last must be WCSPR, and any intermediate elements represent intermediate pairs to trade through (if, for example, a direct pair does not exist).
<br>msg.sender should have already given the router an allowance of at least amount_in_max on the input token.
<br>If the to address is a smart contract, it must have the ability to receive cspr.

Following is the table of parameters.

Parameter Name | Type
---|---
amount_out | U256
amount_in_max | U256
path | Vec<Key>
to | Key
deadline (epoch in milliseconds) | U256

This method **returns** ```amounts: Vector<U256>```
    

- ### swap_exact_tokens_for_cspr <a name="swap_exact_tokens_for_cspr"></a>
Swaps an exact amount of tokens for as much cspr as possible, along the route determined by the path. The first element of path is the input token, the last must be WCSPR, and any intermediate elements represent intermediate pairs to trade through (if, for example, a direct pair does not exist).
<br>If the to address is a smart contract, it must have the ability to receive cspr.

Following is the table of parameters.

Parameter Name | Type
---|---
amount_in | U256
amount_in_min | U256
path | Vec<Key>
to | Key
deadline (epoch in milliseconds) | U256

This method **returns** ```amounts: Vector<U256>```
    
    
- ### swap_cspr_for_exact_tokens <a name="swap_cspr_for_exact_tokens"></a>
Receive an exact amount of tokens for as little CSPR as possible, along the route determined by the path. The first element of path must be WCSPR, the last is the output token and any intermediate elements represent intermediate pairs to trade through (if, for example, a direct pair does not exist).
<br>Leftover CSPR, if any, is returned to msg.sender.

Following is the table of parameters.

Parameter Name | Type
---|---
amount_out | U256
amount_in_max | U256
path | Vec<Key>
to | Key
deadline (epoch in milliseconds) | U256
purse | CLType::Option(Box::new(CLType::URef))

This method **returns** ```amounts: Vector<U256>```


<br><br><br><br>
**Note:** Testcases for methods that involves purses will fail because casper test crate currenly doesnot support purse.
