enum VoteType {
    YesNo,
    StrawPoll,
    Election,
}

pub struct Vote {
    voter_id: String,
}

pub struct Poll {
    pub creator_id: String,
    pub votes: Vec<Vote>,
    pub poll_type: VoteType,
}
