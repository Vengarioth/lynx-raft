#[derive(Debug)]
pub enum Message {
    AppendEntriesRequest(AppendEntriesRequestMessage),
    AppendEntriesResponse(AppendEntriesResponseMessage),
    VoteRequest(VoteRequestMessage),
    VoteResponse(VoteResponseMessage),
}

#[derive(Debug)]
pub struct AppendEntriesRequestMessage {
    // the current leader's term
    term: u64,

    // the current leader's id
    leader_id: u64,

    // index of log entry immediately preceding new ones
    previous_log_index: u64,

    // term of the previous log index entry
    previous_log_term: u64,

    // log entries to store, or empty in case of a heartbeat
    entries: Vec<u8>,

    // current leader's commit index
    leader_commit_index: u64,
}

#[derive(Debug)]
pub struct AppendEntriesResponseMessage {
    // current term
    term: u64,

    // true if follower contained an entry matching previous_log_index and previous_log_term
    success: bool,
}

#[derive(Debug)]
pub struct VoteRequestMessage {
    // the candidate's term
    term: u64,

    // the candidate's id
    candidate_id: u64,

    // the index of the candidate's last log entry
    last_log_index: u64,

    // the term of the candidate's last log entry
    last_log_term: u64,
}

#[derive(Debug)]
pub struct VoteResponseMessage {
    // current term of the recipient
    term: u64,

    // true if the candidate received the vote
    vote_granted: bool,
}
