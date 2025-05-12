# Example Contract "Flipper" 

For building the contract 

```
cargo contract build
```

```
target
  └─ ink
    └─ flipper.contract
    └─ flipper.wasm
    └─ flipper.json
```

- flipper.wasm: This is the raw contract bytecode that will be deployed on-chain.

- flipper.json: The isolated metadata, which is not stored on-chain. It's big and would take up too much space and costs. This file is used by e.g. a dApp user interface to know how to communicate with the on-chain contract.

- flipper.contract: Combines both the contract's bytecode and the metadata. This file is used when you are using a Developer UI like Contracts UI.