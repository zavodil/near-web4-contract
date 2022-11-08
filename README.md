NEAR Web4 Contract Boilerplate 
======

- [Web4 Protocol](https://web4.near.page/) support
- Basic smart contract structure
- Scripts to build local/docker binaries

Live demo: https://near-web4-contract.testnet.page/

How to use
======
Create new repo && download this boilerplate to target folder: 

```
wget https://github.com/zavodil/near-web4-contract/archive/refs/heads/main.zip -O "near-web4-contract-master.zip" && unzip ./"near-web4-contract-master.zip" -d $PWD && rm ./"near-web4-contract-master.zip" && mv -v $PWD/near-web4-contract-main/* $PWD && rm -rf near-web4-contract-main
```

## Quick Start
To run this project:

1. Run `./build.sh` to compile wasm binary fine
2. Register testnet account using https://wallet.testnet.near.org/create (for example, `contract.testnet`)
3. Store your contract_id in the environment variable `export CONTRACT_ID=contract.testnet`
4. Deploy contract to NEAR Blockchain: `near deploy $CONTRACT_ID --wasmFile=./res/web4.wasm -f`
5. Initialize the contract `near call $CONTRACT_ID new '{"owner_id": "'$CONTRACT_ID'"}' --accountId $CONTRACT_ID`
6. Check your contract online on https://$CONTRACT_ID.testnet.page (for example contract.testnet.page)
7. Register account on a mainnet and deploy your contract there, yor app will be available on https://$CONTRACT_ID.near.page

## Gitpod

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/zavodil/near-web4-contract)

Build docker on M1
===
Prepare docker
```
 clone https://github.com/near/near-sdk-rs/pull/720/files
 ./build_docker_m1.sh
```

Run docker buildx `contract-builder`
``` 
 ./build_docker_m1.sh
```

Examples
===
- [Web4 Hackathon example](https://github.com/zavodil/web4-hackathon-example)

Prerequisites
===

Ensure `near-cli` is installed by running:

```
near --version
```

If needed, install `near-cli`:

```
npm install near-cli -g
```

Ensure `Rust` is installed by running:

```
rustc --version
```

If needed, install `Rust`:

```
curl https://sh.rustup.rs -sSf | sh
```

Run the compiler

```
./build_local.sh
```
