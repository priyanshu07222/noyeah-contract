use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};

declare_id!("2S2ztAYPLzQN3McM2jJqNhoycahBMpyEc1tvNLBdR2qv");

#[program]
pub mod noyeah_c {
    use super::*;

    pub fn create_contest(
        ctx: Context<CreateContest>,
        title: String,
        end_time: i64,
        entry_fee: u64,
    ) -> Result<()> {
        let create_contest = &mut ctx.accounts.create_contest;
        let time = Clock::get()?.unix_timestamp;
        create_contest.creator = *ctx.accounts.signer.key;
        create_contest.title = title;
        create_contest.base_entry_price = entry_fee;
        create_contest.start_time = time;
        create_contest.end_time = end_time;
        create_contest.option_yes_pool = 0;
        create_contest.option_no_pool = 0;
        create_contest.total_pool = 0;
        create_contest.yes_participants = 0;
        create_contest.no_participants = 0;
        create_contest.status = ContestStatus::Open;
        Ok(())
    }

    pub fn participate_contest(
        ctx: Context<ParticipateContest>,
        contest: Pubkey,
        amount: u64,
        bidOption: OptionType,
    ) -> Result<()> {
        let contest_acc = &mut ctx.accounts.contest_account;
        let participant_acc = &mut ctx.accounts.participant_account;

        let time = Clock::get()?.unix_timestamp;

        require!(
            contest_acc.status == ContestStatus::Open,
            ErrorCode::ContestClosed
        );
        require!(time < contest_acc.end_time, ErrorCode::ContestClosed);
        require!(
            amount >= contest_acc.base_entry_price,
            ErrorCode::InsufficiantBidAmount
        );

        // a function to calculate price
        // then tranfer the money to the vault
        // change the state of the participant
        // change the betting amount of yes and no

        if contest_acc.status != ContestStatus::Open {
            return Err(ErrorCode::ContestClosed.into());
        }

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title:String)]
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
    #[account(mut)]
    pub contest_account: Account<'info, CreateContestState>,
    /// CHECK: This is a PDA for storing SOL, no need for TokenAccount
    #[account(mut)]
    pub contest_vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FinalizeContest<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub contest_account: Account<'info, CreateContestState>,
    #[account(mut)]
    pub participant: Account<'info, ParticipantState>,
    /// CHECK: PDA for storing SOL
    #[account(mut)]
    pub contest_vault: UncheckedAccount<'info>,
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
    pub total_pool: u64, // can be removed, we will able to find this by yes + no pool
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
    pub option: OptionType,
    pub amount: u64,
    pub price_at_bid: u64,
    pub is_winner: bool,
    // should i store bump
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ContestStatus {
    Open,
    Closed,
}

impl Space for ContestStatus {
    const INIT_SPACE: usize = 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum OptionType {
    Yes,
    No,
}

impl Space for OptionType {
    const INIT_SPACE: usize = 1; // use 1 byte to represent the enum
}

#[error_code]
pub enum ErrorCode {
    #[msg("Contest closed")]
    ContestClosed,
    #[msg("Insufficient bid amount, amount should be greater than or equal to Bid amount")]
    InsufficiantBidAmount,
}
