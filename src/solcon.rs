use anchor_lang::prelude::*;

declare_id!("YourProgramIdHere");

#[program]
pub mod solana_contest {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
        start_date_time: i64,
        end_date_time: i64,
        entry_fee: u64,
        voting_fee: u64,
        winner_percentage: u64,
        number_of_lucky_voters: u64,
    ) -> ProgramResult {
        let contest = &mut ctx.accounts.contest;
        contest.name = name;
        contest.start_date_time = start_date_time;
        contest.end_date_time = end_date_time;
        contest.entry_fee = entry_fee;
        contest.voting_fee = voting_fee;
        contest.winner_percentage = winner_percentage;
        contest.number_of_lucky_voters = number_of_lucky_voters;
        contest.highest_votes = 0;
        Ok(())
    }

    pub fn submit_entry(ctx: Context<SubmitEntry>, image: String) -> ProgramResult {
        let contest = &mut ctx.accounts.contest;

        if !(Clock::get()?.unix_timestamp >= contest.start_date_time && Clock::get()?.unix_timestamp <= contest.end_date_time) {
            return Err(ErrorCode::ContestNotActive.into());
        }

        let submission = Submission {
            wallet: *ctx.accounts.user.key,
            image,
            votes: 0,
        };

        contest.submissions.push(submission);
        Ok(())
    }

    pub fn vote_for_submission(ctx: Context<VoteForSubmission>, submission_index: u64) -> ProgramResult {
        let contest = &mut ctx.accounts.contest;

        if Clock::get()?.unix_timestamp > contest.end_date_time {
            return Err(ErrorCode::ContestNotActive.into());
        }

        if submission_index as usize >= contest.submissions.len() {
            return Err(ErrorCode::SubmissionDoesNotExist.into());
        }

        let submission = &mut contest.submissions[submission_index as usize];
        submission.votes += 1;

        if submission.votes > contest.highest_votes {
            contest.highest_votes = submission.votes;
            contest.winning_submission_indices = vec![submission_index];
        } else if submission.votes == contest.highest_votes {
            contest.winning_submission_indices.push(submission_index);
        }

        Ok(())
    }

    pub fn end_contest(ctx: Context<EndContest>) -> ProgramResult {
        let contest = &mut ctx.accounts.contest;

        if Clock::get()?.unix_timestamp < contest.end_date_time {
            return Err(ErrorCode::ContestNotEnded.into());
        }

        if contest.submissions.is_empty() {
            return Err(ErrorCode::NoSubmissionsMade.into());
        }

        if contest.highest_votes == 0 {
            return Err(ErrorCode::NoVotesCast.into());
        }

        let total_prize = **ctx.accounts.token_account.to_account_info().try_borrow_lamports()?;
        let winner_prize = (total_prize * contest.winner_percentage / 100) / contest.winning_submission_indices.len() as u64;

        for &index in &contest.winning_submission_indices {
            let winner_address = contest.submissions[index as usize].wallet;
            **ctx.accounts.token_account.to_account_info().try_borrow_mut_lamports()? -= winner_prize;
            **ctx.accounts.winner.to_account_info().try_borrow_mut_lamports()? += winner_prize;
        }

        let remaining_prize = total_prize - (winner_prize * contest.winning_submission_indices.len() as u64);
        distribute_prize_to_lucky_voters(ctx, remaining_prize)?;
        Ok(())
    }

    fn distribute_prize_to_lucky_voters(ctx: Context<EndContest>, remaining_prize: u64) -> ProgramResult {
        let contest = &ctx.accounts.contest;

        let number_of_voters = contest.voters.len() as u64;
        let number_of_prize_winners = if contest.number_of_lucky_voters < number_of_voters {
            contest.number_of_lucky_voters
        } else {
            number_of_voters
        };

        let prize_per_lucky_voter = remaining_prize / number_of_prize_winners;
        for _ in 0..number_of_prize_winners {
            let random_index = (Clock::get()?.unix_timestamp % number_of_voters) as usize;
            let lucky_voter = contest.voters[random_index];
            **ctx.accounts.token_account.to_account_info().try_borrow_mut_lamports()? -= prize_per_lucky_voter;
            **ctx.accounts.voter.to_account_info().try_borrow_mut_lamports()? += prize_per_lucky_voter;
        }

        Ok(())
    }

    pub fn update_contest_parameters(
        ctx: Context<UpdateContestParameters>,
        entry_fee: u64,
        voting_fee: u64,
        winner_percentage: u64,
        number_of_lucky_voters: u64,
    ) -> ProgramResult {
        let contest = &mut ctx.accounts.contest;

        if Clock::get()?.unix_timestamp >= contest.start_date_time && Clock::get()?.unix_timestamp <= contest.end_date_time {
            return Err(ErrorCode::ContestNotActive.into());
        }

        contest.entry_fee = entry_fee;
        contest.voting_fee = voting_fee;
        contest.winner_percentage = winner_percentage;
        contest.number_of_lucky_voters = number_of_lucky_voters;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + Contest::LEN)]
    pub contest: Account<'info, Contest>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitEntry<'info> {
    #[account(mut)]
    pub contest: Account<'info, Contest>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct VoteForSubmission<'info> {
    #[account(mut)]
    pub contest: Account<'info, Contest>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct EndContest<'info> {
    #[account(mut)]
    pub contest: Account<'info, Contest>,
    #[account(mut)]
    pub token_account: AccountInfo<'info>,
    #[account(mut)]
    pub winner: AccountInfo<'info>,
    #[account(mut)]
    pub voter: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateContestParameters<'info> {
    #[account(mut)]
    pub contest: Account<'info, Contest>,
    pub user: Signer<'info>,
}

#[account]
pub struct Contest {
    pub name: String,
    pub start_date_time: i64,
    pub end_date_time: i64,
    pub entry_fee: u64,
    pub voting_fee: u64,
    pub winner_percentage: u64,
    pub number_of_lucky_voters: u64,
    pub submissions: Vec<Submission>,
    pub highest_votes: u64,
    pub winning_submission_indices: Vec<u64>,
    pub voters: Vec<Pubkey>,
}

impl Contest {
    const LEN: usize = 64 + 8 + 8 + 8 + 8 + 8 + 8 + 32 + 8 + 8 + (32 * 100); // Adjust as needed
}

#[derive(Clone)]
pub struct Submission {
    pub wallet: Pubkey,
    pub image: String,
    pub votes: u64,
}

#[error]
pub enum ErrorCode {
    #[msg("Contest is not active.")]
    ContestNotActive,
    #[msg("Contest has not ended yet.")]
    ContestNotEnded,
    #[msg("Submission does not exist.")]
    SubmissionDoesNotExist,
    #[msg("You have already voted.")]
    AlreadyVoted,
    #[msg("You have already submitted.")]
    AlreadySubmitted,
    #[msg("No submissions have been made.")]
    NoSubmissionsMade,
    #[msg("No votes have been cast.")]
    NoVotesCast,
    #[msg("Failed to transfer winner prize.")]
    WinnerPrizeTransferFailed,
    #[msg("Failed to transfer prize.")]
    PrizeTransferFailed,
    #[msg("No funds to withdraw.")]
    NoFundsToWithdraw,
    #[msg("Withdrawal failed.")]
    WithdrawalFailed,
}
