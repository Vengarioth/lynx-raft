mod message;
mod state;
mod random;

use self::message::*;
use self::state::*;
use self::random::Random;

#[derive(Debug)]
pub struct Raft {
    configuration: Configuration,
    state: State,
    random: Random,
}

impl Raft {
    pub fn new(configuration: Configuration) -> Raft {
        let mut random = Random::new(configuration.id);
        let state = State::new(random.next_between(configuration.min_election_timeout, configuration.max_election_timeout));

        Raft {
            configuration,
            state,
            random,
        }
    }

    pub fn tick(&mut self, incoming_messages: Vec<Message>) -> Vec<Message> {

        let outgoing_messages = Vec::new();
        self.state.timeout -= 1;

        match self.state.role {
            Role::Follower => {
                if self.state.timeout == 0 {
                    self.start_election();
                }
            },
            Role::Candidate => {
                if self.state.timeout == 0 {
                    self.start_election();
                }
            },
            Role::Leader => {

            }
        }

        outgoing_messages
    }

    pub fn start_election(&mut self) {
        self.state.role = Role::Candidate;
        self.state.timeout = self.get_new_timeout();

        // TODO send out vote requests
    }

    fn get_new_timeout(&mut self) -> u64 {
        self.random.next_between(self.configuration.min_election_timeout, self.configuration.max_election_timeout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_a_follower_it_starts_an_election_after_its_timeout_elapses() {
        // we set the min and max election timeout to the same value to make testing easier
        let config = Configuration::new(1, vec![2, 3], 2, 2);
        let mut raft = Raft::new(config);

        raft.tick(Vec::new());
        raft.tick(Vec::new());

        assert_eq!(raft.state.role, Role::Candidate);
        assert_eq!(raft.state.timeout, 2);
    }

    #[test]
    fn as_a_candidate_it_starts_a_new_election_after_its_election_timeout_elapses() {
        // we set the min and max election timeout to the same value to make testing easier
        let config = Configuration::new(1, vec![2, 3], 2, 2);
        let mut raft = Raft::new(config);

        // trigger an election manually
        raft.start_election();

        raft.tick(Vec::new());
        raft.tick(Vec::new());

        assert_eq!(raft.state.role, Role::Candidate);
        assert_eq!(raft.state.timeout, 2);
    }
}
