# Solana Airdrop Example

This repository contains the code used in this [Solana Tutorials YouTube video](https://www.youtube.com/watch?v=nwS3DhkgHX0&ab_channel=SolanaTutorials). It demonstrates how to send a transaction to transfer SOL in Rust using `solana-test-validator` (local validator).

## Prerequisites

- Follow the instructions to install the [Solana CLI](https://solana.com/docs/intro/installation).

### 1. Start the Local Validator

Open a terminal and start a local Solana validator with the following command:

```
solana-test-validator
```

### 2. Run the Example Code

With the validator running, open another terminal window and execute one of the examples below.

> Both Example 1 and Example 2 are functionally equivalent.
>
> Example 2 is simply an expanded (desugared) version of Example 1.

#### Example 1

To run Example 1, use:

```
cargo run --bin example_1
```

#### Example 2

To run Example 2, use:

```
cargo run --bin example_2
```

**Expected output:**

The output will be similar to the following:

```
from: 5UF3Ycxb8j46CtfgbM99YNfgEV7rjnjh744gm3FMEED3
to: DtpnJNMBmWpU1JYtiwtL2j6brWkzWSAr92peLtQpLJa

Instruction {
    program_id: 11111111111111111111111111111111,
    accounts: [
        AccountMeta {
            pubkey: 5UF3Ycxb8j46CtfgbM99YNfgEV7rjnjh744gm3FMEED3,
            is_signer: true,
            is_writable: true,
        },
        AccountMeta {
            pubkey: DtpnJNMBmWpU1JYtiwtL2j6brWkzWSAr92peLtQpLJa,
            is_signer: false,
            is_writable: true,
        },
    ],
    data: [
        2,
        0,
        0,
        0,
        0,
        225,
        245,
        5,
        0,
        0,
        0,
        0,
    ],
}

Message {
    header: MessageHeader {
        num_required_signatures: 1,
        num_readonly_signed_accounts: 0,
        num_readonly_unsigned_accounts: 1,
    },
    account_keys: [
        5UF3Ycxb8j46CtfgbM99YNfgEV7rjnjh744gm3FMEED3,
        DtpnJNMBmWpU1JYtiwtL2j6brWkzWSAr92peLtQpLJa,
        11111111111111111111111111111111,
    ],
    recent_blockhash: 11111111111111111111111111111111,
    instructions: [
        CompiledInstruction {
            program_id_index: 2,
            accounts: [
                0,
                1,
            ],
            data: [
                2,
                0,
                0,
                0,
                0,
                225,
                245,
                5,
                0,
                0,
                0,
                0,
            ],
        },
    ],
}

Transaction {
    signatures: [
        2wbzWgqFPrSdMxiumJDV4cc3Xs5nXwyQShUr6sQ5wwwTUXyKkunACwz8wAQGNnVaxKyjHYPuiCwUXR9f5THRacgc,
    ],
    message: Message {
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 1,
        },
        account_keys: [
            5UF3Ycxb8j46CtfgbM99YNfgEV7rjnjh744gm3FMEED3,
            DtpnJNMBmWpU1JYtiwtL2j6brWkzWSAr92peLtQpLJa,
            11111111111111111111111111111111,
        ],
        recent_blockhash: HyD1QtWUQquu1F6N7LmUkkD8gnpmMAgpJ859fzAQBcWW,
        instructions: [
            CompiledInstruction {
                program_id_index: 2,
                accounts: [
                    0,
                    1,
                ],
                data: [
                    2,
                    0,
                    0,
                    0,
                    0,
                    225,
                    245,
                    5,
                    0,
                    0,
                    0,
                    0,
                ],
            },
        ],
    },
}

Transaction: https://explorer.solana.com/tx/2wbzWgqFPrSdMxiumJDV4cc3Xs5nXwyQShUr6sQ5wwwTUXyKkunACwz8wAQGNnVaxKyjHYPuiCwUXR9f5THRacgc?cluster=custom
```
