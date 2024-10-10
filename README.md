# error

veronica@Latitude-E5450:~/codigo.io/platform/examples/solana_native/basic_nft/program/generated$ cargo build
   Compiling spl-token-2022 v1.0.0
   Compiling csl_spl_token v0.0.0 (/home/veronica/codigo.io/platform/examples/solana_native/basic_nft/program/libraries/csl_spl_token 0.0.0)
warning: unused import: `std::str::FromStr`
 --> libraries/csl_spl_token 0.0.0/src/cpi.rs:4:5
  |
4 | use std::str::FromStr;
  |     ^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `next_account_info`
 --> libraries/csl_spl_token 0.0.0/src/cpi.rs:7:49
  |
7 | use solana_program::account_info::{AccountInfo, next_account_info};
  |                                                 ^^^^^^^^^^^^^^^^^

warning: unused imports: `AccountMeta` and `Instruction`
 --> libraries/csl_spl_token 0.0.0/src/cpi.rs:9:35
  |
9 | use solana_program::instruction::{AccountMeta, Instruction};
  |                                   ^^^^^^^^^^^  ^^^^^^^^^^^

warning: unused import: `solana_program::system_program`
  --> libraries/csl_spl_token 0.0.0/src/cpi.rs:13:5
   |
13 | use solana_program::system_program;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: arguments to this function are incorrect
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:42:4
    |
42  | ...  &spl_token::instruction::approve(&id(), source_info.key, delegate_info.key, owner_info.key, &...
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  ---------------  -----------------  -------------- expected `Pubkey`, found a different `Pubkey`
    |                                       |      |                |
    |                                       |      |                expected `Pubkey`, found a different `Pubkey`
    |                                       |      expected `Pubkey`, found a different `Pubkey`
    |                                       expected `Pubkey`, found a different `Pubkey`
    |
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:996:8
    |
996 | pub fn approve(
    |        ^^^^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:42:3
    |
41  |     invoke(
    |     ------ arguments to this function are incorrect
42  |         &spl_token::instruction::approve(&id(), source_info.key, delegate_info.key, owner_info.key, &[], amount).unwrap...
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

error[E0308]: arguments to this function are incorrect
    --> libraries/csl_spl_token 0.0.0/src/cpi.rs:76:4
     |
76   | ...  &spl_token::instruction::approve_checked(&id(), source_info.key, mint_info.key,delegate_info.key, owner_info.key, &...
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  ---------------  ------------- -----------------  -------------- expected `Pubkey`, found a different `Pubkey`
     |                                               |      |                |             |
     |                                               |      |                |             expected `Pubkey`, found a different `Pubkey`
     |                                               |      |                expected `Pubkey`, found a different `Pubkey`
     |                                               |      expected `Pubkey`, found a different `Pubkey`
     |                                               expected `Pubkey`, found a different `Pubkey`
     |
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:1269:8
     |
1269 | pub fn approve_checked(
     |        ^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:76:3
    |
75  |     invoke(
    |     ------ arguments to this function are incorrect
76  |         &spl_token::instruction::approve_checked(&id(), source_info.key, mint_info.key,delegate_info.key, owner_info.key, &[], amount, decimals).unwrap...
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

error[E0308]: arguments to this function are incorrect
    --> libraries/csl_spl_token 0.0.0/src/cpi.rs:106:4
     |
106  | ...  &spl_token::instruction::burn(&id(), account_info.key, mint_info.key, owner_info.key,&[...
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  ----------------  -------------  -------------- expected `Pubkey`, found a different `Pubkey`
     |                                    |      |                 |
     |                                    |      |                 expected `Pubkey`, found a different `Pubkey`
     |                                    |      expected `Pubkey`, found a different `Pubkey`
     |                                    expected `Pubkey`, found a different `Pubkey`
     |
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:1117:8
     |
1117 | pub fn burn(
     |        ^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:106:3
    |
105 |     invoke(
    |     ------ arguments to this function are incorrect
106 |         &spl_token::instruction::burn(&id(), account_info.key, mint_info.key, owner_info.key,&[], amount).unwrap...
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

error[E0308]: arguments to this function are incorrect
    --> libraries/csl_spl_token 0.0.0/src/cpi.rs:127:4
     |
127  | ...  &spl_token::instruction::freeze_account(&id(), account_info.key, mint_info.key, owner_info.key, &...
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  ----------------  -------------  -------------- expected `Pubkey`, found a different `Pubkey`
     |                                              |      |                 |
     |                                              |      |                 expected `Pubkey`, found a different `Pubkey`
     |                                              |      expected `Pubkey`, found a different `Pubkey`
     |                                              expected `Pubkey`, found a different `Pubkey`
     |
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:1176:8
     |
1176 | pub fn freeze_account(
     |        ^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:127:3
    |
126 |     invoke(
    |     ------ arguments to this function are incorrect
127 |         &spl_token::instruction::freeze_account(&id(), account_info.key, mint_info.key, owner_info.key, &[]).unwrap...
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

error[E0308]: arguments to this function are incorrect
    --> libraries/csl_spl_token 0.0.0/src/cpi.rs:149:4
     |
149  | ...  &spl_token::instruction::close_account(&id(), account_info.key, destination_info.key, owner_info.key, &...
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  ----------------  --------------------  -------------- expected `Pubkey`, found a different `Pubkey`
     |                                             |      |                 |
     |                                             |      |                 expected `Pubkey`, found a different `Pubkey`
     |                                             |      expected `Pubkey`, found a different `Pubkey`
     |                                             expected `Pubkey`, found a different `Pubkey`
     |
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:1147:8
     |
1147 | pub fn close_account(
     |        ^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:149:3
    |
148 |     invoke(
    |     ------ arguments to this function are incorrect
149 |         &spl_token::instruction::close_account(&id(), account_info.key, destination_info.key, owner_info.key, &[]).unwrap...
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

error[E0308]: arguments to this function are incorrect
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:182:4
    |
182 | ...  &spl_token::instruction::initialize_account(&id(), account_info.key, mint_info.key, owner_info.key).u...
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  ----------------  -------------  -------------- expected `Pubkey`, found a different `Pubkey`
    |                                                  |      |                 |
    |                                                  |      |                 expected `Pubkey`, found a different `Pubkey`
    |                                                  |      expected `Pubkey`, found a different `Pubkey`
    |                                                  expected `Pubkey`, found a different `Pubkey`
    |
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:832:8
    |
832 | pub fn initialize_account(
    |        ^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:182:3
    |
181 |     invoke(
    |     ------ arguments to this function are incorrect
182 |         &spl_token::instruction::initialize_account(&id(), account_info.key, mint_info.key, owner_info.key).unwrap...
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

error[E0308]: arguments to this function are incorrect
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:210:4
    |
210 | ...  &spl_token::instruction::initialize_account2(&id(), account_info.key, mint_info.key, &owner).u...
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  ----------------  -------------  ------ expected `Pubkey`, found a different `Pubkey`
    |                                                   |      |                 |
    |                                                   |      |                 expected `Pubkey`, found a different `Pubkey`
    |                                                   |      expected `Pubkey`, found a different `Pubkey`
    |                                                   expected `Pubkey`, found a different `Pubkey`
    |
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:856:8
    |
856 | pub fn initialize_account2(
    |        ^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:210:3
    |
209 |     invoke(
    |     ------ arguments to this function are incorrect
210 |         &spl_token::instruction::initialize_account2(&id(), account_info.key, mint_info.key, &owner).unwrap...
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

error[E0308]: arguments to this function are incorrect
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:233:4
    |
233 | ...  &spl_token::instruction::initialize_account3(&id(), account_info.key, mint_info.key, &owner).u...
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  ----------------  -------------  ------ expected `Pubkey`, found a different `Pubkey`
    |                                                   |      |                 |
    |                                                   |      |                 expected `Pubkey`, found a different `Pubkey`
    |                                                   |      expected `Pubkey`, found a different `Pubkey`
    |                                                   expected `Pubkey`, found a different `Pubkey`
    |
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:882:8
    |
882 | pub fn initialize_account3(
    |        ^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:233:3
    |
232 |     invoke(
    |     ------ arguments to this function are incorrect
233 |         &spl_token::instruction::initialize_account3(&id(), account_info.key, mint_info.key, &owner).unwrap...
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

error[E0308]: arguments to this function are incorrect
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:266:4
    |
266 | ...  &spl_token::instruction::initialize_mint(&id(), mint_info.key, &mint_authority, f...
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  -------------  --------------- expected `Pubkey`, found a different `Pubkey`
    |                                               |      |
    |                                               |      expected `Pubkey`, found a different `Pubkey`
    |                                               expected `Pubkey`, found a different `Pubkey`
    |
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: expected `Pubkey`, found a different `Pubkey`
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:266:83
    |
266 | ...info.key, &mint_authority, freeze_authority.as_ref(), decimals).unwrap(),
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:777:8
    |
777 | pub fn initialize_mint(
    |        ^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:266:3
    |
265 |     invoke(
    |     ------ arguments to this function are incorrect
266 |         &spl_token::instruction::initialize_mint(&id(), mint_info.key, &mint_authority, freeze_authority.as_ref(), decimals).unwrap...
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

error[E0308]: arguments to this function are incorrect
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:291:4
    |
291 | ...  &spl_token::instruction::initialize_mint2(&id(), mint_info.key, &mint_authority, f...
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  -------------  --------------- expected `Pubkey`, found a different `Pubkey`
    |                                                |      |
    |                                                |      expected `Pubkey`, found a different `Pubkey`
    |                                                expected `Pubkey`, found a different `Pubkey`
    |
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: expected `Pubkey`, found a different `Pubkey`
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:291:84
    |
291 | ...info.key, &mint_authority, freeze_authority.as_ref(), decimals).unwrap(),
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:806:8
    |
806 | pub fn initialize_mint2(
    |        ^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:291:3
    |
290 |     invoke(
    |     ------ arguments to this function are incorrect
291 |         &spl_token::instruction::initialize_mint2(&id(), mint_info.key, &mint_authority, freeze_authority.as_ref(), decimals).unwrap...
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

error[E0308]: arguments to this function are incorrect
    --> libraries/csl_spl_token 0.0.0/src/cpi.rs:320:4
     |
320  | ...  &spl_token::instruction::mint_to(&id(), mint_info.key, assoc_token_account_info.key, owner_info.key, &...
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  -------------  ----------------------------  -------------- expected `Pubkey`, found a different `Pubkey`
     |                                       |      |              |
     |                                       |      |              expected `Pubkey`, found a different `Pubkey`
     |                                       |      expected `Pubkey`, found a different `Pubkey`
     |                                       expected `Pubkey`, found a different `Pubkey`
     |
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:1087:8
     |
1087 | pub fn mint_to(
     |        ^^^^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:320:3
    |
319 |     invoke(
    |     ------ arguments to this function are incorrect
320 |         &spl_token::instruction::mint_to(&id(), mint_info.key, assoc_token_account_info.key, owner_info.key, &[], amount).unwrap...
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

error[E0308]: arguments to this function are incorrect
    --> libraries/csl_spl_token 0.0.0/src/cpi.rs:339:4
     |
339  | ...  &spl_token::instruction::revoke(&id(), source_info.key, owner_info.key, &[])....
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  ---------------  -------------- expected `Pubkey`, found a different `Pubkey`
     |                                      |      |
     |                                      |      expected `Pubkey`, found a different `Pubkey`
     |                                      expected `Pubkey`, found a different `Pubkey`
     |
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:1026:8
     |
1026 | pub fn revoke(
     |        ^^^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:339:3
    |
338 |     invoke(
    |     ------ arguments to this function are incorrect
339 |         &spl_token::instruction::revoke(&id(), source_info.key, owner_info.key, &[]).unwrap...
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

error[E0308]: arguments to this function are incorrect
    --> libraries/csl_spl_token 0.0.0/src/cpi.rs:372:4
     |
372  | ...  &spl_token::instruction::set_authority(&id(), mint_info.key, new_authority.as_ref(), authority, owner_info.key, &...
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  -------------                                     -------------- expected `Pubkey`, found a different `Pubkey`
     |                                             |      |
     |                                             |      expected `Pubkey`, found a different `Pubkey`
     |                                             expected `Pubkey`, found a different `Pubkey`
     |
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
note: expected `Pubkey`, found a different `Pubkey`
    --> libraries/csl_spl_token 0.0.0/src/cpi.rs:372:64
     |
372  | ...ority(&id(), mint_info.key, new_authority.as_ref(), authority, owner_info.key, ...
     |                                ^^^^^^^^^^^^^^^^^^^^^^
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:1053:8
     |
1053 | pub fn set_authority(
     |        ^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:372:3
    |
371 |     invoke(
    |     ------ arguments to this function are incorrect
372 |         &spl_token::instruction::set_authority(&id(), mint_info.key, new_authority.as_ref(), authority, owner_info.key, &[]).unwrap...
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

error[E0308]: arguments to this function are incorrect
    --> libraries/csl_spl_token 0.0.0/src/cpi.rs:393:4
     |
393  |         &spl_token::instruction::sync_native(&id(), account_info.key).unwrap(),
     |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  ---------------- expected `Pubkey`, found a different `Pubkey`
     |                                              |
     |                                              expected `Pubkey`, found a different `Pubkey`
     |
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:1364:8
     |
1364 | pub fn sync_native(
     |        ^^^^^^^^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:393:3
    |
392 |     invoke(
    |     ------ arguments to this function are incorrect
393 |         &spl_token::instruction::sync_native(&id(), account_info.key).unwrap(),
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

error[E0308]: arguments to this function are incorrect
    --> libraries/csl_spl_token 0.0.0/src/cpi.rs:414:4
     |
414  | ...  &spl_token::instruction::thaw_account(&id(), account_info.key, mint_info.key, owner_info.key, &...
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  ----------------  -------------  -------------- expected `Pubkey`, found a different `Pubkey`
     |                                            |      |                 |
     |                                            |      |                 expected `Pubkey`, found a different `Pubkey`
     |                                            |      expected `Pubkey`, found a different `Pubkey`
     |                                            expected `Pubkey`, found a different `Pubkey`
     |
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:1205:8
     |
1205 | pub fn thaw_account(
     |        ^^^^^^^^^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:414:3
    |
413 |     invoke(
    |     ------ arguments to this function are incorrect
414 |         &spl_token::instruction::thaw_account(&id(), account_info.key, mint_info.key, owner_info.key, &[]).unwrap...
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

error[E0308]: arguments to this function are incorrect
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:442:4
    |
442 | ...  &spl_token::instruction::transfer(&id(), source_info.key, destination_info.key, authority_info.key, &...
    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  ---------------  --------------------  ------------------ expected `Pubkey`, found a different `Pubkey`
    |                                        |      |                |
    |                                        |      |                expected `Pubkey`, found a different `Pubkey`
    |                                        |      expected `Pubkey`, found a different `Pubkey`
    |                                        expected `Pubkey`, found a different `Pubkey`
    |
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
    = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
    |
87  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
    |
96  | pub struct Pubkey(pub(crate) [u8; 32]);
    | ^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:966:8
    |
966 | pub fn transfer(
    |        ^^^^^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:442:3
    |
441 |     invoke(
    |     ------ arguments to this function are incorrect
442 |         &spl_token::instruction::transfer(&id(), source_info.key, destination_info.key, authority_info.key, &[], amount).unwrap...
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

error[E0308]: arguments to this function are incorrect
    --> libraries/csl_spl_token 0.0.0/src/cpi.rs:478:4
     |
478  | ...  &spl_token::instruction::transfer_checked(&id(), source_info.key, mint_info.key, destination_info.key, authority_info.key, &...
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -----  ---------------  -------------  --------------------  ------------------ expected `Pubkey`, found a different `Pubkey`
     |                                                |      |                |              |
     |                                                |      |                |              expected `Pubkey`, found a different `Pubkey`
     |                                                |      |                expected `Pubkey`, found a different `Pubkey`
     |                                                |      expected `Pubkey`, found a different `Pubkey`
     |                                                expected `Pubkey`, found a different `Pubkey`
     |
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
     = note: `Pubkey` and `Pubkey` have similar names, but are actually distinct types
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/pubkey.rs:87:1
     |
87   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
note: `Pubkey` is defined in crate `solana_program`
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/pubkey.rs:96:1
     |
96   | pub struct Pubkey(pub(crate) [u8; 32]);
     | ^^^^^^^^^^^^^^^^^
     = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
    --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-4.0.2/src/instruction.rs:1235:8
     |
1235 | pub fn transfer_checked(
     |        ^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> libraries/csl_spl_token 0.0.0/src/cpi.rs:478:3
    |
477 |     invoke(
    |     ------ arguments to this function are incorrect
478 |         &spl_token::instruction::transfer_checked(&id(), source_info.key, mint_info.key, destination_info.key, authority_info.key, &[], amount, decimals).unwrap...
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Instruction`, found a different `Instruction`
    |
    = note: `Instruction` and `Instruction` have similar names, but are actually distinct types
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-2.0.13/src/instruction.rs:332:1
    |
332 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
note: `Instruction` is defined in crate `solana_program`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/instruction.rs:330:1
    |
330 | pub struct Instruction {
    | ^^^^^^^^^^^^^^^^^^^^^^
    = note: perhaps two different versions of crate `solana_program` are being used?
note: function defined here
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/solana-program-1.17.14/src/program.rs:132:8
    |
132 | pub fn invoke(instruction: &Instruction, account_infos: &[AccountInfo]) -> ProgramR...
    |        ^^^^^^

For more information about this error, try `rustc --explain E0308`.
warning: `csl_spl_token` (lib) generated 4 warnings
error: could not compile `csl_spl_token` (lib) due to 34 previous errors; 4 warnings emitted
warning: build failed, waiting for other jobs to finish...
error[E0277]: can't compare `solana_program::pubkey::Pubkey` with `spl_memo::solana_program::pubkey::Pubkey`
  --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-2022-1.0.0/src/extension/memo_transfer/mod.rs:46:20
   |
46 |         program_id == &spl_memo::id() || program_id == &spl_memo::v1::id()
   |                    ^^ no implementation for `solana_program::pubkey::Pubkey == spl_memo::solana_program::pubkey::Pubkey`
   |
   = help: the trait `PartialEq<spl_memo::solana_program::pubkey::Pubkey>` is not implemented for `solana_program::pubkey::Pubkey`, which is required by `&solana_program::pubkey::Pubkey: PartialEq<&spl_memo::solana_program::pubkey::Pubkey>`
   = help: the trait `PartialEq` is implemented for `solana_program::pubkey::Pubkey`
   = help: for that trait implementation, expected `solana_program::pubkey::Pubkey`, found `spl_memo::solana_program::pubkey::Pubkey`
   = note: required for `&solana_program::pubkey::Pubkey` to implement `PartialEq<&spl_memo::solana_program::pubkey::Pubkey>`

error[E0277]: can't compare `solana_program::pubkey::Pubkey` with `spl_memo::solana_program::pubkey::Pubkey`
  --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-2022-1.0.0/src/extension/memo_transfer/mod.rs:46:53
   |
46 |         program_id == &spl_memo::id() || program_id == &spl_memo::v1::id()
   |                                                     ^^ no implementation for `solana_program::pubkey::Pubkey == spl_memo::solana_program::pubkey::Pubkey`
   |
   = help: the trait `PartialEq<spl_memo::solana_program::pubkey::Pubkey>` is not implemented for `solana_program::pubkey::Pubkey`, which is required by `&solana_program::pubkey::Pubkey: PartialEq<&spl_memo::solana_program::pubkey::Pubkey>`
   = help: the trait `PartialEq` is implemented for `solana_program::pubkey::Pubkey`
   = help: for that trait implementation, expected `solana_program::pubkey::Pubkey`, found `spl_memo::solana_program::pubkey::Pubkey`
   = note: required for `&solana_program::pubkey::Pubkey` to implement `PartialEq<&spl_memo::solana_program::pubkey::Pubkey>`

error[E0599]: no function or associated item named `get_packed_len` found for struct `spl_token::state::Account` in the current scope
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-2022-1.0.0/src/state.rs:310:57
    |
310 |                         .get(spl_token::state::Account::get_packed_len())
    |                                                         ^^^^^^^^^^^^^^ function or associated item not found in `Account`
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `Pack` which provides `get_packed_len` is implemented but not in scope; perhaps you want to import it
    |
3   + use spl_memo::solana_program::program_pack::Pack;
    |

error[E0277]: can't compare `solana_program::pubkey::Pubkey` with `spl_memo::solana_program::pubkey::Pubkey`
   --> /home/veronica/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spl-token-2022-1.0.0/src/lib.rs:111:62
    |
111 |     if spl_token_program_id != &id() && spl_token_program_id != &spl_token::id() {
    |                                                              ^^ no implementation for `solana_program::pubkey::Pubkey == spl_memo::solana_program::pubkey::Pubkey`
    |
    = help: the trait `PartialEq<spl_memo::solana_program::pubkey::Pubkey>` is not implemented for `solana_program::pubkey::Pubkey`, which is required by `&solana_program::pubkey::Pubkey: PartialEq<&spl_memo::solana_program::pubkey::Pubkey>`
    = help: the trait `PartialEq` is implemented for `solana_program::pubkey::Pubkey`
    = help: for that trait implementation, expected `solana_program::pubkey::Pubkey`, found `spl_memo::solana_program::pubkey::Pubkey`
    = note: required for `&solana_program::pubkey::Pubkey` to implement `PartialEq<&spl_memo::solana_program::pubkey::Pubkey>`

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `spl-token-2022` (lib) due to 4 previous errors
veronica@Latitude-E5450:~/codigo.io/platform/examples/solana_native/basic_nft/program/generated$ 
