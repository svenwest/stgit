use git2::RepositoryState;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum Error {
    #[error("Git2: {0}")]
    Git(#[from] git2::Error),

    #[error("JSON deserialize error: {0}")]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Format(#[from] std::fmt::Error),

    #[error(transparent)]
    PatchName(#[from] crate::patchname::Error),

    #[error(transparent)]
    PatchRange(#[from] crate::patchrange::Error),

    #[error("could not execute `git`: {0}")]
    GitExecute(std::io::Error),

    #[error("`git {0}`: {1}")]
    GitCommand(String, String),

    #[error(transparent)]
    PathSpec(#[from] crate::pathspec::Error),

    #[error("patch `{0}` already exists")]
    PatchAlreadyExists(crate::patchname::PatchName),

    #[error("patch `{0}` does not exist")]
    PatchDoesNotExist(crate::patchname::PatchName),

    #[error("not on branch, HEAD is detached")]
    HeadDetached,

    #[error("not on branch, HEAD points at `{0}`")]
    HeadNotBranch(String),

    #[error("StGit cannot be initialized on unborn branch: {0}")]
    UnbornBranch(String),

    #[error("branch `{0}` not found")]
    BranchNotFound(String),

    #[error("invalid branch name `{0}`")]
    InvalidBranchName(String),

    #[error("invalid StGit revision `{0}`")]
    InvalidRevision(String),

    #[error("revision not found `{0}`")]
    RevisionNotFound(String),

    #[error("branch `{0}` not initialized")]
    StackNotInitialized(String),

    #[error("branch `{0}` already initialized")]
    StackAlreadyInitialized(String),

    #[error("stack metadata not found")]
    StackMetadataNotFound,

    #[error("non-UTF-8 branch name `{0}`")]
    NonUtf8BranchName(String),

    #[error("non-UTF-8 {0} `{1}`")]
    NonUtf8Argument(String, String),

    #[error("file `{0}` contains non-UTF-8 data")]
    NonUtf8File(String),

    #[error("patch description contains non-UTF-8 data")]
    NonUtf8PatchDescription,

    #[error("{0}")]
    NonUtf8Signature(String),

    #[error("non-UTF-8 alias name `{0}` in {1}")]
    NonUtf8AliasName(String, String),

    #[error("non-UTF-8 alias value for `{0}` in {1}")]
    NonUtf8AliasValue(String, String),

    #[error("non-UTF-8 commit message from `{0}`")]
    NonUtf8Message(String),

    #[error("bad alias for `{0}`: {1}")]
    BadAlias(String, String),

    #[error("recursive alias `{0}`")]
    RecursiveAlias(String),

    #[error("while expanding shell alias `{0}`: `{1}`: {2}")]
    ExecuteAlias(String, String, String),

    #[error("{0}")]
    MissingSignature(String),

    #[error("failed to parse patch description: {0}")]
    ParsePatchDescription(String),

    #[error("resolve outstanding conflicts first")]
    OutstandingConflicts,

    #[error("invalid name and email `{0}`")]
    InvalidNameEmail(String),

    #[error("invalid date `{0}` from `{1}`")]
    InvalidDate(String, String),

    #[error("problem with the editor `{0}`")]
    EditorFail(String),

    #[error("`{0}` hook: {1}")]
    Hook(String, String),

    #[error(
        "HEAD and stack top are not the same. \
         This can happen if you modify the branch with git. \
         See `stg repair --help` for next steps to take."
    )]
    StackTopHeadMismatch,

    #[error("Index not clean. Use `refresh` or `reset --hard`")]
    DirtyIndex,

    #[error("Worktree not clean. Use `refresh` or `reset --hard`")]
    DirtyWorktree,

    #[error("Complete the in-progress `{0}` before trying again.")]
    ActiveRepositoryState(String),

    #[error("{0}\nCommand aborted (all changes rolled back)")]
    TransactionAborted(String),

    #[error("{0}")]
    TransactionHalt(String),

    // TODO: lowercase
    #[error("No patches applied")]
    NoAppliedPatches,

    #[error("Not enough patches applied")]
    NotEnoughPatchesApplied,

    #[error("No unapplied patches")]
    NoUnappliedPatches,

    #[error("{0}")]
    CheckoutConflicts(String),

    #[error("{0}")]
    Generic(String),
}

pub(crate) fn repo_state_to_str(state: RepositoryState) -> &'static str {
    match state {
        RepositoryState::Clean => "clean",
        RepositoryState::Merge => "merge",
        RepositoryState::Revert | RepositoryState::RevertSequence => "revert",
        RepositoryState::CherryPick | RepositoryState::CherryPickSequence => "cherry-pick",
        RepositoryState::Bisect => "bisect",
        RepositoryState::Rebase => "rebase",
        RepositoryState::RebaseInteractive => "interactive rebase",
        RepositoryState::RebaseMerge => "rebase merge",
        RepositoryState::ApplyMailbox => "apply mailbox",
        RepositoryState::ApplyMailboxOrRebase => "rebase or apply mailbox",
    }
}
