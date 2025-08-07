Humanhash System Specification
Overview
Humanhash is a Decentralized Proof of Personhood (PoP) platform using biometrics, Zero-Knowledge Proofs (ZKPs) via BitSNARK, PoPChain for identity commitments, and oracles (sureBits, Chainlink) for external data (KYC, payments). It supports three service tiers (BASIC, LIVE, FULL) for identity enrollment and verification.
User Journey

Enrollment: User submits biometric data via multi-platform app, system generates zk-SNARK proof (BitSNARK), commits hash to PoPChain, sureBits provides KYC data. A unique sequence code is generated for logging.
Client App: BiometricUpload.tsx or native modules capture facial scan, send to /identity/enroll.
Backend: Biometric service (biometric.rs) processes data, generates proof, commits to PoPChain, logs event with sequence code. Oracle service queries sureBits.
Output: "Enrollment successful, ID hash: 0x123..., Sequence Code: TX-ENR-UUID-2025-08-07-HASH"


Authentication: User submits proof, system verifies with BitSNARK, sureBits confirms external data. Sequence code logs the transaction.
Client App: Identity.tsx or native modules submit proof to /identity/verify.
Backend: System service verifies proof, oracle fetches data, logs action with sequence code.
Output: "Access granted, Sequence Code: TX-AUTH-UUID-2025-08-07-HASH"


Revocation/Update: User updates data, system regenerates proof and updates PoPChain. Sequence code logs the update.
Client App: Identity.tsx or native modules manage updates.
Backend: Biometric and PoPChain services update commitments, log action with sequence code.
Output: "Identity updated, Sequence Code: TX-UPDATE-UUID-2025-08-07-HASH"



Verifier Journey

Receive Proof: Verifier gets zk-SNARK proof via QR or manual input, system verifies with BitSNARK. Sequence code logs the verification.
Verifier App: ProofVerifier.tsx submits proof to /identity/verify.
Backend: System service verifies against PoPChain, logs with sequence code.
Output: "Proof valid, Sequence Code: TX-VER-UUID-2025-08-07-HASH"


Query Data: Oracle fetches KYC/payment data from sureBits. Sequence code logs the query.
Verifier App: ComplianceCheck.tsx queries /oracle/kyc or /oracle/payment.
Backend: Oracle service fetches data, logs with sequence code.
Output: "User KYC verified, Sequence Code: TX-QUERY-UUID-2025-08-07-HASH"


Log/Audit: System logs verification to PoPChain. Sequence code enables reporting.
Verifier App: AuditLog.tsx displays logs.
Backend: PoPChain logs event with sequence code.
Output: "Verification logged at 0x456..., Sequence Code: TX-LOG-UUID-2025-08-07-HASH"



User/Client App Architecture

Platform Layer: Multi-platform support (iOS, Android, Web PWA).
iOS: Swift/Objective-C, React Native, FaceTec iOS SDK (FaceTecManager.swift).
Android: Kotlin/Java, React Native, FaceTec Android SDK (FaceTecManager.kt).
Web PWA: React, TypeScript, WebAssembly bridge, WebRTC for camera, IndexedDB storage (client/src/components/BiometricUpload.tsx, Identity.tsx, OracleStatus.tsx).
Applesque Style: Clean, modular, user-centric design with declarative UI (SwiftUI-like), secure storage (Keychain/Secure Enclave on iOS/Android, IndexedDB on Web), performance optimization, and intuitive UX (minimalist interfaces, smooth animations).
Dependencies: @types/react, axios, facetec-sdk, qrcode-generator, react-native.


Business Logic Layer: React Native/Web shared core (Redux).
Enrollment Manager: Tier-specific workflows (BASIC, LIVE, FULL), biometric capture with FaceTec, quality assurance, sequence code generation for logging.
Verification Engine: Live verification requests, response handling, retry logic.
Identity Management: State management (Redux), secure storage (AsyncStorage, encrypted), sync management, privacy controls, export/backup.
Dependencies: redux, redux-persist, redux-persist-transform-encrypt, react-native.


Backend Interaction:
Biometric Service (biometric/src/main.rs): Processes biometrics with FaceTec/MegaMatcher, generates BitSNARK zk-SNARK proofs, commits to PoPChain, generates sequence code for logging.
System Service (system/src/main.rs): Verifies proofs.
Oracle Service (oracle/src/main.rs): Queries sureBits for KYC/payment data.
PoPChain Service: Manages blockchain commitments and logs with sequence code.


Privacy: BitSNARK generates zk-SNARK proofs for biometric data.
Deployment: Docker container, deployed via RayCluster (kubernetes/raycluster.yaml).

Verifier App Architecture

Platform Layer: Multi-platform support (iOS, Android, Web PWA).
iOS: Swift/Objective-C, React Native, POS SDK integration (e.g., Zebra EMDK).
Android: Kotlin/Java, React Native, POS SDK integration (e.g., Honeywell SDK).
Web PWA: React, TypeScript, WebRTC for camera, IndexedDB for storage (ProofVerifier.tsx, ComplianceCheck.tsx, AuditLog.tsx).
Applesque Style: Clean, modular, user-centric design with declarative UI, secure storage, performance optimization, and intuitive UX (minimalist interfaces, smooth transitions).
POS Scanning: Integration with existing POS hardware (Zebra/Honeywell scanners) for QR codes, using SDKs for native apps and Web APIs for PWA.
Dependencies: @types/react, axios, react-qr-reader, POS-sdk (e.g., zebra-emdk).


Business Logic Layer: React Native/Web shared core (Redux).
ProofVerifier.tsx: Handles QR scanner (camera interface, QR decode, instant results) or manual input (text input, validation, search history), submits proofs to /identity/verify.
ComplianceCheck.tsx: Queries /oracle/kyc or /oracle/payment for compliance data.
AuditLog.tsx: Displays real-time verification status, progress tracking, and results with retry handling, generates sequence code for logging.
Dependencies: redux, redux-persist, redux-persist-transform-encrypt, react-native.


Backend Interaction:
System Service (system/src/main.rs): Verifies BitSNARK proofs against PoPChain commitments, logs with sequence code.
Oracle Service (oracle/src/main.rs): Fetches sureBits data for KYC/payment.
PoPChain Service: Logs verification events with sequence code.


Privacy: BitSNARK ensures no biometric data exposure.
Deployment: Docker container, deployed via RayCluster (kubernetes/raycluster.yaml).
API Integration: REST endpoints (/identity/verify, /oracle/kyc) for third-party systems.

Implementation Notes

BitSNARK: Add to biometric.rs and system.rs for zk-SNARK proof generation/verification to ensure privacy.
sureBits: Integrate into oracle/src/main.rs for KYC/payment data feeds.
POS Scanning: Add POS SDKs (Zebra/Honeywell) to native layers for QR scanning in verifiers.
Reporting: Generate unique sequence code (TX-ACTION-UUID-TIMESTAMP-HASH) for all events, log to PoPChain, expose /report/events endpoint for users/verifiers/outlets to query logs.
CI/CD: Update ci-cd.yaml to build and test new features, including POS integrations.
RayCluster: Leverage existing RayCluster (kubernetes/raycluster.yaml) for scalable deployment.

