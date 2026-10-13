//! Pure decision logic for the signer: given a request and the current state, decide
//! whether to sign. No I/O, no clock (time is always an argument), integer-only math.
//!
//! Types only for now; no decision logic has been written yet.
#![forbid(unsafe_code)]

/// A request to sign, as received from the backend.
///
/// TODO(interfaces): the request format is defined by the owner-exported interfaces;
/// this is a placeholder and carries no protocol details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignRequest {}

/// Outcome of evaluating a [`SignRequest`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// Policy permits signing.
    Allow,
    /// Policy forbids signing, with the reason.
    Refuse(RefuseReason),
}

/// Why a request was refused. Logged as a code, never with message bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefuseReason {
    /// The request or transaction could not be parsed.
    ParseFailure,
    /// Program ID is not on the key's allowlist.
    ProgramNotAllowed,
    /// Instruction is not on the key's allowlist.
    InstructionNotAllowed,
    /// Sequence number is not strictly greater than the last persisted one.
    SeqNotIncreasing,
    /// Batches-per-minute ceiling reached.
    BatchRateExceeded,
    /// confirm_pass-per-hour ceiling reached.
    ConfirmPassRateExceeded,
    /// Verifier heartbeat is missing, invalid or stale (dead-man switch).
    HeartbeatStale,
    /// The halt flag is set.
    Halted,
}

/// The policy configuration and state a decision is evaluated against.
///
/// TODO(interfaces): fields (allowlists, ceilings, counters) are not defined yet.
#[derive(Debug, Clone, Default)]
pub struct Policy {}
