/// Raft membership roles
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Role {
    Follower,
    Candidate,
    Leader,
}

/// Single node configuration
#[derive(Debug)]
pub struct Configuration {
    /// id of the local node
    pub id: u64,

    /// list of peer nodes and their ids
    pub peers: Vec<u64>,

    /// minimum timeout until follower node starts an election
    pub min_election_timeout: u64,
    
    /// maximum timeout until follower node starts an election
    pub max_election_timeout: u64,
}

impl Configuration {
    pub fn new(id: u64, peers: Vec<u64>, min_election_timeout: u64, max_election_timeout: u64) -> Configuration {
        Configuration {
            id,
            peers,
            min_election_timeout,
            max_election_timeout,
        }
    }
}

/// Persistent state on all servers (Updated on stable storage before responding to RPCs)
#[derive(Debug)]
pub struct PersistentState {
    /// latest term server has seen (initialized to 0 on first boot, increases monotonically)
    pub current_term: u64,

    /// candidateId that received vote in current term (or None if none)
    pub voted_for: Option<u64>,

    /// log entries; each entry contains  command for the state machine and term when the entry was received by leader (first index is 1)
    pub log: Vec<u64>,
}

impl PersistentState {
    pub fn new() -> PersistentState {
        PersistentState {
            current_term: 0,
            voted_for: None,
            log: vec![0],
        }
    }
}

/// Volatile state on all servers
#[derive(Debug)]
pub struct VolatileState {
    /// index of highest log entry known to be committed (initialized to 0, increases monotonically)
    pub commit_index: u64,

    /// index of highest log entry applied to state machine (initialized to 0, increases monotonically)
    pub last_applied: u64,
}

impl VolatileState {
    pub fn new() -> VolatileState {
        VolatileState {
            commit_index: 0,
            last_applied: 0,
        }
    }
}

/// Volatile state on leaders (Reinitialized after election)
#[derive(Debug)]
pub struct VolatileLeaderState {
    /// for each server, index of the next log entry to send to that server (initialized to leader last log index + 1)
    pub next_index: Vec<u64>,

    /// for each server, index of highest log entry known to be replicated on server (initialized to 0, increases monotonically)
    pub match_index: Vec<u64>,
}

impl VolatileLeaderState {
    /// TODO: correct initialization
    pub fn new() -> VolatileLeaderState {
        VolatileLeaderState {
            next_index: Vec::new(),
            match_index: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub persistent_state: PersistentState,
    pub volatile_state: VolatileState,
    pub volatile_leader_state: VolatileLeaderState,

    /// current membership role of the raft node (initialized to Follower)
    pub role: Role,

    /// remaining ticks until an action according to the node's membership role is due (initialized to a new election timeout)
    pub timeout: u64,
}

impl State {
    pub fn new(timeout: u64) -> State {
        State {
            persistent_state: PersistentState::new(),
            volatile_state: VolatileState::new(),
            volatile_leader_state: VolatileLeaderState::new(),

            role: Role::Follower,
            timeout,
        }
    }
}
