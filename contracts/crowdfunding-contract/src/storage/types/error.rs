use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    ContractInitialized = 0,
    ContractNotInitialized = 1,
    MathOverflow = 2,
    MathUnderflow = 3,
    CampaignNotFound = 4,
    CampaignGoalExceeded = 5,
    ContributionBelowMinimum = 6,
    AmountMustBePositive = 7,
    CampaignGoalNotReached = 8,
    ContributionNotFound = 9,
    CampaignAlreadyExists = 10,
    ProofNotFound = 11,
    InvalidGoalAmount = 12,
    InvalidMinDonation = 13,
    InvalidMilestoneAmount = 14,
    MilestoneAmountNotIncreasing = 15,
    MilestoneNotFound = 16,
    MilestoneAlreadyCompleted = 17,
    InsufficientFundsForMilestone = 18,
    MilestoneNotInSequence = 19,
    MilestoneNotCompleted = 20,
    CannotWithdrawFutureMilestone = 21,
    NoFundsToWithdraw = 22,
}
