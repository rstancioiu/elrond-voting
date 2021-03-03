#![no_std]
#![allow(clippy::too_many_arguments)]

imports!();

mod poll_info;
mod status;

use poll_info::PollInfo;
use status::Status;

const DEFAULT_DEADLINE_INTERVAL: u64 = 60 * 60 * 24 * 7;

#[elrond_wasm_derive::contract(CommunityVotingImpl)]
pub trait CommunityVoting {
    #[init]
    fn init(& self) {}

    #[endpoint]
    fn create(
        & self,
        poll_name: BoxedBytes,
        question: BoxedBytes,
        choices: BoxedBytes,
        opt_deadline: Option<u64>,
        opt_vote_limit: Option<u32>,
    ) -> SCResult<()> {
        require!(! poll_name.is_empty(), "Poll name can't be empty!");

        let timestamp = self.get_block_timestamp();
        require!(
            self.get_status(& poll_name, & timestamp) == Status::Inactive,
            "Community vote is already active!"
        );
    
        let deadline = opt_deadline.unwrap_or_else(|| timestamp + DEFAULT_DEADLINE_INTERVAL);
        require!(deadline > timestamp, "Deadline can't be in the past!");
    
        require!(choices.len() > 0, "No choices have been submitted!");
        let votes_distribution: Vec<u32> = self.initiliaze_votes_distribution(& choices);

        let poll_info = PollInfo {
            question, choices, votes_distribution, deadline, opt_vote_limit
        };

        self.set_poll_info(& poll_name, & poll_info);

        Ok(())
    }

    fn initiliaze_votes_distribution(& self, choices: & BoxedBytes) -> Vec<u32> {
        let nb_choices = choices.as_slice()[0] as u8;
        let mut votes_distribution: Vec<u32> = Vec::new();
        for _index in (0..nb_choices).rev() {
            votes_distribution.push(0u32);
        }
        votes_distribution
    }

    #[endpoint]
    fn vote(& self, poll_name: & BoxedBytes, choice: u32) -> SCResult<()> {
        match self.status(& poll_name) {
            Status::Inactive => sc_error!("Community vote is currently inactive!"),
            Status::Running => { return self.vote_in_poll(& poll_name, choice); },
            Status::Ended =>  sc_error!("Community vote has finished! Check the results.")
        }
    }

    fn vote_in_poll(& self, poll_name: & BoxedBytes, choice: u32) -> SCResult<()> {    
        let mut poll_info = self.get_poll_info(& poll_name);
        let caller_address = self.get_caller();

        // TODO: Create method is_choice_valid and improve error.
        let selected_choice: usize = choice as usize;
        let maximum_nb_choices: usize = poll_info.votes_distribution.len();
        require!(
            selected_choice < maximum_nb_choices,
            "The selected choice needs to be between 0 and nb_answers - 1!"
        );

        // TODO: Create method has_caller_already_voted.
        let vote_count = self.get_current_vote_count(& poll_info.votes_distribution);
        for vote_index in (0..vote_count).rev() {
            require!(
                self.get_vote_owner(poll_name, vote_index) != caller_address,
                "A voter can vote only once!"
            );
        }

        self.set_vote_owner(poll_name, vote_count, & caller_address);

        poll_info.votes_distribution[selected_choice] += 1;
        self.set_poll_info(poll_name, &poll_info);

        Ok(())
    }

    fn get_current_vote_count(& self, votes_distribution: & Vec<u32>) -> u32 {
        let mut current_vote_count = 0 as u32;
        for votes in votes_distribution {
            current_vote_count += votes;
        }
        current_vote_count
    }

    #[view]
    fn get_results(& self, poll_name: & BoxedBytes) -> Option<PollInfo> {
        let poll_status: Status = self.status(&poll_name);
        if poll_status == Status::Inactive {
            return None;
        }
        if poll_status == Status::Running {
            return None;
        }
        Some(self.get_poll_info(& poll_name))
    }

    #[view]
    fn status(& self, poll_name: & BoxedBytes) -> Status {
        let timestamp = self.get_block_timestamp();
        self.get_status(& poll_name, & timestamp)
    }

    fn get_status(& self, poll_name: & BoxedBytes, block_timestamp: & u64) -> Status {
        if self.is_empty_poll_info(poll_name) {
            return Status::Inactive;
        }

        let poll_info = self.get_poll_info(& poll_name);
        let vote_count = self.get_current_vote_count(& poll_info.votes_distribution);
        match poll_info.opt_vote_limit {
            Some(vote_limit) => {
                if vote_limit == vote_count {
                    return Status::Ended;
                }
            },
            None => {}
        }
        if (* block_timestamp) > poll_info.deadline {
            return Status::Ended;
        }

        Status::Running
    }

    #[storage_set("pollInfo")]
    fn set_poll_info(& self, poll_name: & BoxedBytes, poll_info: & PollInfo);

    #[view(pollInfo)]
    #[storage_get("pollInfo")]
    fn get_poll_info(& self, poll_name: & BoxedBytes) -> PollInfo;

    #[storage_is_empty("pollInfo")]
    fn is_empty_poll_info(& self, poll_name: & BoxedBytes) -> bool;

    #[storage_set("voteOwner")]
    fn set_vote_owner(& self, pool_name: & BoxedBytes, vote_id: u32, vote_owner: & Address);

    #[storage_get("voteOwner")]
    fn get_vote_owner(& self, pool_name: & BoxedBytes, vote_id: u32) -> Address;
}
