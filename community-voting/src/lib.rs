#![no_std]
#![allow(clippy::too_many_arguments)]

imports!();

mod poll_info;
mod status;

use poll_info::PollInfo;
use status::Status;

const DEFAULT_DEADLINE_INTERVAL: u64 = 60 * 60 * 24 * 7;
const DEFAULT_VOTE_LIMIT: u32 = 2000;

#[elrond_wasm_derive::contract(CommunityVotingImpl)]
pub trait CommunityVoting {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn create(
        &self,
        poll_name: BoxedBytes,
        question: BoxedBytes,
        choices: BoxedBytes,
        opt_deadline: Option<u64>,
        opt_vote_limit: Option<u32>,
    ) -> SCResult<()> {
        self.create_voting_poll(
            poll_name,
            question,
            choices,
            opt_deadline,
            opt_vote_limit
        )
    }

    fn create_voting_poll(
        &self,
        poll_name: BoxedBytes,
        question: BoxedBytes,
        choices: BoxedBytes,
        opt_deadline: Option<u64>,
        opt_vote_limit: Option<u32>
    ) -> SCResult<()> {
        require!(! poll_name.is_empty(), "Poll name can't be empty!");

        require!(
            self.status(&poll_name) == Status::Inactive,
            "Community vote is already active!"
        );
    
        let timestamp = self.get_block_timestamp();
        let deadline = opt_deadline.unwrap_or_else(|| timestamp + DEFAULT_DEADLINE_INTERVAL);
        require!(deadline > timestamp, "Deadline can't be in the past!");
    
        let vote_limit = opt_vote_limit.unwrap_or_else(|| DEFAULT_VOTE_LIMIT);
        require!(choices.len() > 0, "No choices have been submitted");
        let nb_choices = choices.as_slice()[0] as u8;
        let mut votes_distribution: Vec<u32> = Vec::new();
        for _index in (0..nb_choices).rev() {
            votes_distribution.push(0u32);
        }

        let poll_info = PollInfo {
            question,
            choices,
            votes_distribution,
            deadline,
            vote_limit
        };

        self.set_poll_info(&poll_name, &poll_info);

        Ok(())
    }

    #[endpoint]
    fn vote(&self, poll_name: BoxedBytes, choice: u32) -> SCResult<()> {
        match self.status(&poll_name) {
            Status::Inactive => sc_error!("Community vote is currently inactive."),
            Status::Running => self.vote_in_poll(&poll_name, choice),
            Status::Ended =>  sc_error!("Community vote finished! Check the results.")
        }
    }

    fn vote_in_poll(&self, poll_name: &BoxedBytes, choice: u32) -> SCResult<()> {    
        let mut poll_info = self.get_poll_info(&poll_name);
        let caller_address = self.get_caller();

        let selected_choice: usize = choice as usize;
        let maximum_nb_choices: usize = poll_info.votes_distribution.len();

        require!(
            selected_choice < maximum_nb_choices,
            "The selected choice needs to be between 0 and nb_answers - 1"
        );
        let vote_count = self.get_current_vote_count(& poll_info.votes_distribution);

        for vote_index in (0..vote_count).rev() {
            require!(
                self.get_vote_owner(poll_name, vote_index) != caller_address,
                "A voter can vote only once"
            );
        }

        self.set_vote_owner(poll_name, vote_count, & caller_address);

        poll_info.votes_distribution[selected_choice] += 1;
        self.set_poll_info(poll_name, &poll_info);

        Ok(())
    }

    fn get_current_vote_count(&self, votes_distribution: & Vec<u32>) -> u32 {
        let mut current_vote_count = 0 as u32;
        for votes in votes_distribution {
            current_vote_count += votes;
        }
        current_vote_count
    }

    #[endpoint]
    fn get_results(&self, poll_name: BoxedBytes) -> Option<PollInfo> {
        let poll_status: Status = self.status(&poll_name);
        if poll_status == Status::Inactive {
            return None;
        }
        if poll_status == Status::Running {
            return None;
        }
        return Some(self.get_poll_info(&poll_name));
    }

    #[view]
    fn status(&self, poll_name: &BoxedBytes) -> Status {
        if self.is_empty_poll_info(poll_name) {
            return Status::Inactive;
        }

        let poll_info = self.get_poll_info(& poll_name);
        let vote_count = self.get_current_vote_count(& poll_info.votes_distribution);
        if (self.get_block_timestamp() > poll_info.deadline) || (vote_count == poll_info.vote_limit) {
            return Status::Ended;
        }

        Status::Running
    }

    // storage

    #[storage_set("pollInfo")]
    fn set_poll_info(&self, poll_name: &BoxedBytes, poll_info: &PollInfo);

    #[view(pollInfo)]
    #[storage_get("pollInfo")]
    fn get_poll_info(&self, poll_name: &BoxedBytes) -> PollInfo;

    #[storage_is_empty("pollInfo")]
    fn is_empty_poll_info(&self, poll_name: &BoxedBytes) -> bool;

    #[storage_set("voteOwner")]
    fn set_vote_owner(&self, pool_name: &BoxedBytes, vote_id: u32, vote_owner: &Address);

    #[storage_get("voteOwner")]
    fn get_vote_owner(&self, pool_name: &BoxedBytes, vote_id: u32) -> Address;
}
