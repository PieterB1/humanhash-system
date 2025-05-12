# HumanHash System
Decentralized Proof of Personhood (PoP) system with biometrics, ZKPs, and blockchain.

## Features
- Multimodal biometrics (Neurotechnology MegaMatcher, FaceTec Zoom) with liveness detection (ISO/IEC 30107).
- ZKPs for privacy.
- PoPChain for identity commitments.
- Oracles for KYC data.
- Cross-chain smart contracts.
- Cloud-agnostic with HashiCorp Vault.
- Chaos testing and sharded architecture.

## Setup
1. Install Rust, Node.js, Docker, PostgreSQL.
2. Clone: `git clone https://github.com/pieterb1/humanhash-system.git`.
3. Run: `docker-compose up --build`.
4. Access: `http://localhost:3000`.

## Deployment
- Build: `docker build -t myrepo/humanhash-client:1.0 .`.
- Deploy: `helm install humanhash ./helm/humanhash`.

## License
MIT License

