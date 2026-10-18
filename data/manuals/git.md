# `git`

[//]: # ( cSpell: ignore conflictstyle gitdir gpgsign safecrlf zdiff3 )

## Configuration

### General

A general `.gitconfig` in your home directory could look like this:

```ini
# -----------------------------------------------
# ----  GitConfig  ------------------------------
# ----  https://git-scm.com/docs/git-config  ----
# -----------------------------------------------

[core]
    autocrlf = input
    safecrlf = true
    eol = lf

[init]
    defaultBranch = main

[merge]
    conflictStyle = zdiff3

[push]
    autoSetupRemote = true
    default = simple

[filter "lfs"]
    clean = git-lfs clean -- %f
    smudge = git-lfs smudge -- %f
    process = git-lfs filter-process
    required = true
```

### Use `delta` as a Pager

```ini
[core]
    pager = delta

[interactive]
    diffFilter = delta --color-only

[delta]
    line-numbers = true
    true-color = auto
    hyperlinks = true
```

### Signing Commits

```ini
[commit]
    gpgsign = true

[gpg]
    format = ssh

[user]
    signingKey = key::<YOUR SSH PUBLIC KEY>

[gpg "ssh"]
    allowedSignersFile = ~/<PATH TO FILE>
```

An `allowedSignersFile` looks (roughly) like this:

```txt
<YOUR E-MAIL ADDRESS> <YOUR SSH PUBLIC KEY>
```

### User-Specific Settings

If you maintain projects on different platforms and different users, you can conditionally enable them:

```ini
# -----------------------------------------------
# ----  User-Specific Settings  -----------------
# -----------------------------------------------

# default configuration
[user]
    name = ...
    email = ...
    signingKey = ...

[includeIf "gitdir:**/git/hub/"]
    path = .gitconfig.github.ini

[includeIf "gitdir:**/git/forgejo/"]
    path = .gitconfig.forgejo.ini

[includeIf "gitdir:**/git/ea/"]
    path = .gitconfig.gitlab.ini
```

And create the corresponding files (e.g., `~/.gitconfig.github.ini`):

```ini
[user]
    name = ...
    email = ...
    signingKey = ...

[gpg "ssh"]
    allowedSignersFile = ...

[url "ssh://git@github.com"]
    insteadOf = https://github.com

# If you use the GitHub CLI, you may also
# uncomment the following settings:
#[credential "https://github.com"]
#    helper =
#    helper = !/usr/bin/gh auth git-credential
#[credential "https://gist.github.com"]
#    helper =
#    helper = !/usr/bin/gh auth git-credential
```

> [!WARNING]
>
> Always specify a default configuration if you are working with Remote Development. When in remote targets, other files next to `.gitconfig` are very likely not "forwarded" properly.

## Altering the History

### Shorten the History

[Source](https://stackoverflow.com/questions/11687899/remove-cut-off-gits-revision-commit-history).

> [!CAUTION]
>
> Ensure to have no tags that point to commits that will be "deleted"!

Assume the history `A <–– B <–– C <–– D <–– E <–– F` and we want to delete everything before D, i.e., `A <–– B <–– C`. Execute

```bash
SHA_SUM_OF_D=

# create a new branch with D's content
git checkout --orphan temp "${SHA_SUM_OF_D}"
git commit

# rebase everything else onto the temp branch
git rebase --onto temp "${SHA_SUM_OF_D}" master

# clean up
git checkout master
git branch -d temp

# delete
git reflog expire --expire=now --all
git gc --prune=now
```

### Re-Signing Commits

[Source 1](https://superuser.com/questions/397149/can-you-gpg-sign-old-commits). [Source 2](https://docs.github.com/en/authentication/managing-commit-signature-verification/telling-git-about-your-signing-key).

This rebases everything until development (or any hash) and you don't have to copy and paste after every commit:

```bash
$ HASH=$(git rev-parse --verify HEAD)
$ git rebase --exec 'git commit --amend --no-edit --no-verify --gpg-sign' --interactive "${HASH}"
Successfully rebased and updated ...
```

It may be necessary to specify the key you want to use for singing: `--gpg-sign=<KEY>`. Finally, you will need to do a force-push over the branch. Make sure you set the correct signing key beforehand:

### Adding a File to LFS That Is Currently Not But Should Be

[Source](https://stackoverflow.com/questions/46704572/git-error-encountered-7-files-that-should-have-been-pointers-but-werent)

```bash
git lfs migrate import --no-rewrite "broken file.jpg" "another broken file.png" ...
```
