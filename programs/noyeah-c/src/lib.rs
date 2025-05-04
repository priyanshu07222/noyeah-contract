use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};

declare_id!("2S2ztAYPLzQN3McM2jJqNhoycahBMpyEc1tvNLBdR2qv");

#[program]
pub mod noyeah_c {
    use super::*;

    pub fn create_contest(
        ctx: Context<CreateContest>,
        _title: String,
        _start_time: i64,
        _end_time: i64,
        _entry_fee: u64,
    ) -> Result<()> {
        let _acc = &ctx.accounts;
        Ok(())
    }

    pub fn participate_contest(ctx: Context<ParticipateContest>, _contest: Pubkey) -> Result<()> {
        let _context = &ctx.accounts;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateContest<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer=signer,
        space= 8 + CreateContestState::INIT_SPACE,
        seeds=[b"contest", title.as_bytes()],
        bump,
    )]
    pub create_contest: Account<'info, CreateContestState>,
    /// CHECK: This is the vault account for contest
    #[account(
        seeds=[b"vault", create_contest.key().as_ref()],
        bump
    )]
    pub contest_vault_account: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct ParticipateContest<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space= 8 + ParticipantState::INIT_SPACE,
        seeds=[b"bid", payer.key.as_ref(), contest_account.key().as_ref()],
        bump
    )]
    pub participant_account: Account<'info, ParticipantState>,
    #[account(
        mut,
        seeds=[b"contest", title.as_ref()],
        bump
    )]
    pub contest_account: Account<'info, CreateContestState>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct CreateContestState {
    pub creator: Pubkey,
    #[max_len(200)]
    pub title: String,
    pub start_time: i64,
    pub end_time: i64,
    pub base_entry_price: u64, // start_entry fee then it change dynamically
    pub option_yes_pool: u64,
    pub option_no_pool: u64,
    pub total_pool: u64,
    pub yes_participants: u64,
    pub no_participants: u64,
    pub status: ContestStatus,
    // should i store bump??
}

#[account]
#[derive(InitSpace)]
pub struct ParticipantState {
    pub participant: Pubkey,
    pub contest: Pubkey,
    pub answer: OptionAnswer,
    pub amount: u64,
    pub is_winner: bool,
    // should i store bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ContestStatus {
    Open,
    Closed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum OptionAnswer {
    YES,
    NO,
    PENDING,
}

impl Space for ContestStatus {
    const INIT_SPACE: usize = 1; // use 1 byte to represent the enum
}

impl Space for OptionAnswer {
    const INIT_SPACE: usize = 1; // use 1 byte to represent the enum
}
