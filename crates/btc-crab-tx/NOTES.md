#### WIF Format
Wallet Input Format (WIF) is an encoding format for private keys meant to be human readable using the same Base58 encoding as addresses.
Creating WIF:
1. `0x80` for mainnet or `0xef` for testnet
2. Encode the secret in 32-byte big endian
3. If the SEC format used for the public key addresses was compressed, add a suffix of `0x01`
4. Concatenate 1 || 2 || 3
5. Hash256(step 4) and get first 4 bytes
6. Concatenate 4 || 5


#### Transaction Components
1. Version
2. Inputs
3. Outputs
4. Locktime

##### Version
Bitcoin transactions also have version numbers.
- Most Bitcoin transactions are version 1 but transactions using opcode `OP_CHECKSEQUENCEVERIFY` defined in `BIP0112` use version greater than 1.
- The version is encoded into bytes a little-endian u32 byte array

##### Inputs
- A Bitcoin transaction inputs are spending the outputs of a previous transaction
- Each input needs two things:
  - A reference to bitcoins you received previously
  - Proof that these are yours to spend using ECDSA signatures that owners of the private keys can produce.
- Inputs field can contain more than one input