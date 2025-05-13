use anchor_lang::{prelude::*};

declare_id!("2S2ztAYPLzQN3McM2jJqNhoycahBMpyEc1tvNLBdR2qv");

#[program]
pub mod noyeah_c {
    use anchor_lang::solana_program::{program::{invoke, invoke_signed}, system_instruction};

    use super::*;

    pub fn create_contest(
        ctx: Context<CreateContest>,
        title: String,
        end_time: i64,
        entry_fee: u64,
    ) -> Result<()> {
        let create_contest = &mut ctx.accounts.create_contest;
        create_contest.creator = *ctx.accounts.signer.key;
        create_contest.title = title;
        create_contest.yes_entry_price = entry_fee;
        create_contest.no_entry_price = entry_fee;
        create_contest.start_time = Clock::get()?.unix_timestamp;
        create_contest.end_time = end_time;
        create_contest.option_yes_pool = 0;
        create_contest.option_no_pool = 0;
        create_contest.yes_participants = 0;
        create_contest.no_participants = 0;
        create_contest.status = ContestStatus::Open;
        create_contest.winner_count = 0;
        create_contest.total_pool = 0;
        
        Ok(())
    }

    pub fn participate_contest(
        ctx: Context<ParticipateContest>,
        amount: u64,
        bid_option: OptionType,
    ) -> Result<()> {
        let contest_acc = &mut ctx.accounts.contest_account;
        let participant_acc = &mut ctx.accounts.participant_account;

        let time = Clock::get()?.unix_timestamp;

        require!(
            contest_acc.status == ContestStatus::Open,
            ErrorCode::ContestClosed
        );
        require!(time < contest_acc.end_time, ErrorCode::ContestClosed);
        
        match bid_option {
            OptionType::Yes => {
                require!(
                    amount >= contest_acc.yes_entry_price,
                    ErrorCode::InsufficiantBidAmount
                );
                contest_acc.yes_participants += 1;
                contest_acc.option_yes_pool += amount;
                contest_acc.total_pool += amount;
                participant_acc.price_at_bid = contest_acc.yes_entry_price;
            }
            OptionType::No => {
                require!(
                    amount >= contest_acc.no_entry_price,
                    ErrorCode::InsufficiantBidAmount
                );
                contest_acc.no_participants += 1;
                contest_acc.option_no_pool += amount;
                contest_acc.total_pool += amount;
                participant_acc.price_at_bid = contest_acc.no_entry_price;
            }
        }

        participant_acc.participant = *ctx.accounts.payer.key;
        participant_acc.contest = contest_acc.key();
        participant_acc.is_winner = false;
        participant_acc.amount = amount;
        participant_acc.option = bid_option.clone();
        participant_acc.has_claimed = false;
        // this should change because price is changing
        

        invoke(
            &system_instruction::transfer(
                &ctx.accounts.payer.key(),
                &ctx.accounts.contest_vault.key(),
                amount,
            ),
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.contest_vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        let new_price = calculate_dynamic_price(&contest_acc, bid_option.clone());

        match bid_option {
            OptionType::Yes => contest_acc.yes_entry_price = new_price,
            OptionType::No => contest_acc.no_entry_price = new_price,
        }

        // a function to calculate price -> done
        // then tranfer the money to the vault
        // change the state of the participant
        // change the betting amount of yes and no
        Ok(())
    }
    
    pub fn resolve_contest(ctx: Context<Resolve>, answer: OptionType) -> Result<()>{
        require!(*ctx.accounts.payer.owner == ctx.accounts.contest.creator, ErrorCode::OnlyCreatorCanCallThis);
        require!(Clock::get()?.unix_timestamp > ctx.accounts.contest.end_time, ErrorCode::ContestNotEnded);
        require!(ctx.accounts.contest.status == ContestStatus::Open, ErrorCode::AlreadyResolved);
        let contest = &mut ctx.accounts.contest;
        contest.correct_answer = answer.clone();
        contest.status = ContestStatus::Resolved;
        match answer {
            OptionType::Yes => {
                contest.winner_count = contest.yes_participants;
                },
            OptionType::No => {
                contest.winner_count = contest.no_participants;
            }
        }
        Ok(())
    }

    pub fn finalize_contest(
        ctx: Context<FinalizeContest>,
    ) -> Result<()> {
        let contest = &mut ctx.accounts.contest_account.clone();
        let participant = &mut ctx.accounts.participant;
        require!(
            contest.status == ContestStatus::Resolved,
            ErrorCode::ContestNotResolvedYet
        );
        require!(
            Clock::get()?.unix_timestamp > contest.end_time,
            ErrorCode::ContestNotEnded
        );
        require!(participant.has_claimed == false, ErrorCode::AlreadyClaimed);
        require!(contest.winner_count > 0 , ErrorCode::WinnerCountShouldBeGreaterThanZero);
        // make sure winner don't call this again and again

        if participant.option == contest.correct_answer {
            participant.is_winner = true;

            // Calculate reward
            let reward =
                (participant.amount * contest.total_pool) / (participant.price_at_bid * contest.winner_count);
            
            let (_contest_vault_pda, bump) = Pubkey::find_program_address(
                        &[b"vault", ctx.accounts.contest_account.key().as_ref()],
                        ctx.program_id,
                    );
            
            let vpubkey = ctx.accounts.contest_account.key();
            
            let seeds: &[&[&[u8]]] = &[&[b"vault", vpubkey.as_ref(), &[bump]]];

            // Transfer reward
            // invoke_sign
            invoke_signed(
                &system_instruction::transfer(
                    &ctx.accounts.contest_vault.key(),
                    &ctx.accounts.payer.key(),
                    reward,
                ),
                &[
                    ctx.accounts.contest_vault.to_account_info(),
                    ctx.accounts.payer.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                seeds
            )?;
            
            if contest.winner_count == 1 {
                contest.status = ContestStatus::Closed;
            }
            contest.winner_count -= 1;
            contest.total_pool -= reward;
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
// #[instruction(title:String)]
pub struct ParticipateContest<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space= 8 + ParticipantState::INIT_SPACE,
        seeds=[b"bid", payer.key().as_ref(), contest_account.key().as_ref()],
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
pub struct Resolve<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub contest: Account<'info, CreateContestState>
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
    #[max_len(80)]
    pub title: String,
    pub start_time: i64,
    pub end_time: i64,
    pub yes_entry_price: u64,
    pub no_entry_price: u64,
    pub option_yes_pool: u64,
    pub option_no_pool: u64,
    pub total_pool: u64,
    pub yes_participants: u64,
    pub no_participants: u64,
    pub correct_answer: OptionType,
    pub status: ContestStatus,
    pub winner_count: u64,
    pub bump: u8
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
    pub has_claimed: bool,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ContestStatus {
    Open,
    Closed,
    Resolved
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
    #[msg("The contest has not yet ended.")]
    ContestNotEnded,
    #[msg("Only creator can call this instruction")]
    OnlyCreatorCanCallThis,
    #[msg("Already resolved")]
    AlreadyResolved,
    #[msg("Contest not resolved yet")]
    ContestNotResolvedYet,
    #[msg("Already Claimed")]
    AlreadyClaimed,
    #[msg("Winner count should be greater than 0")]
    WinnerCountShouldBeGreaterThanZero
}

pub fn calculate_dynamic_price(contest: &Account<CreateContestState>, option: OptionType) -> u64 {
    match option {
        OptionType::Yes => {
            contest.yes_entry_price * (1 + (contest.option_yes_pool as u64 / contest.total_pool.max(1)))
        }
        OptionType::No => {
            contest.no_entry_price * (1 + (contest.option_no_pool as u64 / contest.total_pool.max(1)))
        }
    }
}
