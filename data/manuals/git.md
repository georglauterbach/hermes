# `git`

## References, Revisions, Object IDs & Commit IDs

| Term | Explanation | Example | Technicalities |
| :--- | :---------- | :------ | :------------- |
| `ref` / `reference` | a user-friendly, human-readable name that points directly to a specific commit (hash) | `main`, `v1.8.5` | a plain text (pointer) file stored inside your repository's hidden `.git/refs/` directory containing a commit hash |
| `rev` / `revision` | an expression or language syntax used to identify a commit or a group of commits | `main`, `f7e23ac`, `HEAD~1` | broad query string that `git` parses to find a specific point in history, a syntax you type into a command line on the fly |
| Object ID (OID) | `git` doesn't just hash commits, it also hashes files (blobs) and directories (trees); every single one of these items gets a unique hash, the "Object ID" | | internal term |
| Commit ID | the specific term used when an OID belongs exclusively to a commit object | | 40 (SHA1) or 64 (SHA256) hexadecimal characters long, depending on the hash function |

> Think of it like an address book: a `ref` is an official contact entry saved in your phone under a specific name (e.g., "Mom"). A revision is any way you describe how to reach a person, which could be saying their name ("Mom"), typing their raw phone number (a commit hash), or saying "Mom's neighbor" (`main~1`).

Every ref can be used as a revision, but not every revision is a ref.

When you type `git show main`, `main` is a physical ref that `git` resolves into a revision. However, when you type `git show HEAD~2`, `HEAD~2` is a revision expression (meaning "two commits before current"). There is no file on your computer named `HEAD~2`; `git` computes the calculation dynamically.

### Common Types of References

1. Branches (`refs/heads/`)
    - mutable (sliding) pointers
    - when you are on a branch and make a new commit, `git` automatically updates this `ref` file to point to the new commit hash
2. Tags (`refs/tags/`)
    - immutable (static) pointers
    - they act as permanent bookmarks in your history (like release versions) and do not change even if you add more commits later
3. Remotes (`refs/remotes/`)
    - read-only bookmarks that track the last known state of branches on a remote server (like Forgejo)
    - they only update when you run commands like `git fetch` or `git pull`
4. Special Case `HEAD`
    - special file located at `.git/HEAD`
    - a symbolic reference, meaning it usually points to another `ref` rather than a commit hash
    - tracks whichever branch you currently have checked out
    - example: if you are working on `main`, the `HEAD` file simply contains `ref: refs/heads/main`
