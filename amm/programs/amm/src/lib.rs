pub mod constants;
pub mod error;
pub mod contexts;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use contexts::*;
pub use state::*;

declare_id!("JAddDtwQn7oytJ4PkvGGaTdTEWBG7usXxYbjZN1q4dRv");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
