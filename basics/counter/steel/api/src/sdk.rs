use steel::*;

use crate::prelude::*;

pub fn initialize(signer: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(counter_pda().0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Initialize {}.to_bytes(),
    }
}

pub fn increment(signer: Pubkey) -> Instruction {
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(counter_pda().0, false),
        ],
        data : Increment {}.to_bytes(),
    }
}