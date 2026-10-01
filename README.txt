# Provenance Machine

Cryptographic provenance infrastructure for the **Crypto World’s Fair by Colosseum**.

Provenance Machine lets researchers, developers, and organizations create verifiable records for digital artifacts without putting the underlying files on-chain.

Files are hashed locally. The resulting cryptographic commitment is anchored on Solana. Later, the original artifact can be hashed again and independently compared against its on-chain provenance record.

**local artifact → SHA-256 hash → Solana commitment → independent verification**

The system is designed for research papers, datasets, software releases, experimental results, archives, and other digital artifacts where establishing version history and integrity matters.
