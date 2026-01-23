# `git`

[//]: # ( cSpell: ignore safecrlf gpgsign conflictstyle zdiff3 )
[//]: # ( cSpell: ignore gitdir )

## Configuration

### General

A general `.gitconfig` in your home directory could look like this:

```ini
# -----------------------------------------------
# ----  GitConfig  ------------------------------
# ----  https://git-scm.com/docs/git-config  ----
# -----------------------------------------------

# -----------------------------------------------
# ----  General  --------------------------------
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

You can configure additional useful options:

1. Signing commits using SSH keys (instead of GPG keys):

    ```ini
    [commit]
        gpgsign = true

    [gpg]
        format = ssh
    ```

    Also add the key [`signingKey`](https://git-scm.com/docs/git-config#Documentation/git-config.txt-usersigningKey) with your SSH public key to your `[user]` section and [`allowedSignersFile`](https://git-scm.com/docs/git-config#Documentation/git-config.txt-gpgsshallowedSignersFile) to the `gpg.ssh` section:

    ```ini
    [user]
        signingKey = key::ssh-...

    [gpg "ssh"]
        allowedSignersFile = ...
    ```

    An `allowedSignersFile` looks (roughly) like this:

    ```txt
    <YOUR GITHUB E-MAIL ADDRESS> <YOUR SSH PUBLIC KEY>
    ```

2. Using a pager for rendering the diff:

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

### User-Specific

If you maintain projects on different platforms and different users, you can conditionally enabled them. Add the following to `~/.gitconfig`:

```ini
# -----------------------------------------------
# ----  User-Specific  --------------------------
# -----------------------------------------------

# default configuration
[user]
    name = ...
    email = ...
    signingKey = ...

[includeIf "gitdir:**/git/hub/"]
    path = .gitconfig.github

[includeIf "gitdir:**/git/lab/"]
    path = .gitconfig.gitlab

[includeIf "gitdir:**/git/ea/"]
    path = .gitconfig.gitlab
```

And create the corresponding files (e.g., `~/.gitconfig.github`):

```ini
# vim: syntax=gitconfig

[user]
    name = ...
    email = ...
    signingKey = ...

[gpg "ssh"]
    allowedSignersFile = ...

[url "ssh://git@github.com"]
    insteadOf = https://github.com
```

> [!WARNING]
>
> Always specify a default configuration if you are working with Remote Development. When in remote targets, other files next to `.gitconfig` are very likely not "forwarded" properly.

## Altering the History

### Replacing Commit Name and E-Mail

[Source](https://stackoverflow.com/questions/2919878/git-rewrite-previous-commit-usernames-and-emails)

```bash
pip3 install git-filter-repo
OLD_NAME=
NEW_NAME=
OLD_MAIL=
NEW_MAIL=44545919+aendeavor@users.noreply.github.com
git-filter-repo --name-callback "return name.replace(b'${OLD_NAME}', b'${NEW_NAME}')" --email-callback "return email.replace(b'${OLD_MAIL}', b'${NEW_MAIL}')"

GITHUB_USERNAME=
REPO_NAME=
git remote add "${REPO_NAME}" "git@github.com:${GITHUB_USERNAME}/${REPO_NAME}.git"
```

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

## Adding a File to LFS That Is Currently Not But Should Be

[Source](https://stackoverflow.com/questions/46704572/git-error-encountered-7-files-that-should-have-been-pointers-but-werent)

```bash
git lfs migrate import --no-rewrite "broken file.jpg" "another broken file.png" ...
```
