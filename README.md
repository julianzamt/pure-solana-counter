# Native Solana Rust Counter
A very simple counter on Native Solana Rust for research purposes.  
  
It includes both the client and the source code.  

# Usage
1 - Build: cargo build-sbf (build-bpf for older versions of solana cli). 
2 - Deploy: solana program deploy target/deploy/pure-solana-counter.so --url localhost (for deploying to a local validator). 
3 - Select in cli/index.ts main fn the ix to run. 
4 - Execute: npx ts-node index.ts. 
