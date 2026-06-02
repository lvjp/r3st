# Contributing to r3st

We would love for you to contribute to the `r3st` and help make it even better than it is today! As
a contributor, we would like you to follow the guidelines.

## Coding Rules

To ensure consistency throughout the source code, keep these rules in mind as you are working:

* All features or bug fixes **must be tested** by one or more unit-tests.
* All public API methods **must be documented**. (Details TBC).
* Source code formatting is verified with golangci-lint. All code is wraped at **100 characters**.

## Commit Message Guidelines

We have very precise rules over how our git commit messages can be formatted. This leads to **more
readable messages** that are easy to follow when looking through the **project history**. But also,
we use the git commit messages to **generate the r3st change log**.

### Commit Message Format

As we follow the [Conventional Commits][conventional-commits] specification, each commit message
consists of a **header**, a **body** and a **footer**. The header has a special format that includes
a **type**, a **scope** and a **subject**:

```text
<type>(<scope>): <subject>
<BLANK LINE>
<body>
<BLANK LINE>
<footer>
```

The **header** is mandatory and the **scope** of the header is optional.

Any line of the commit message cannot be longer 100 characters! This allows the message to be easier
to read on GitHub as well as in various git tools.

The footer should contain a [closing reference to an issue][github-issue-closing] if any.

Samples:

```text
docs(changelog): correct spelling
```

```text
fix: prevent racing of requests

Introduce a request id and a reference to latest request. Dismiss incoming responses other than
from latest request.

Remove timeouts which were used to mitigate the racing issue but are obsolete now.

Refs: #123
```

### Revert

If the commit reverts a previous commit, it should begin with `revert:`, followed by the header of
the reverted commit. In the body it should say: `This reverts commit <hash>.`, where the hash is the
SHA of the commit being reverted.

### Type

Should be one of the following:

* **build**: Changes that affect the build system or external dependencies
* **ci**: Changes to our CI configuration files and scripts
* **docs**: Documentation only changes
* **feat**: A new feature
* **fix**: A bug fix
* **perf**: A code change that improves performance
* **refactor**: A code change that neither fixes a bug nor adds a feature
* **style**: Changes that do not affect the meaning of the code (white-space, formatting, missing
  semi-colons, etc)
* **test**: Adding missing tests or correcting existing tests

### Scope

The scope should be the name of the go module affected as perceived by the person reading the
changelog generated from commit messages.  
For submodules of `cmd/`, `pkg/` and `internal/`, the submodules name should be used instead.

There are currently a few exceptions to the "use module name" rule:

* **changelog**: used for updating the release notes in CHANGELOG.md
* none/empty string: useful for `style`, `test` and `refactor` changes that are done across all
  packages (e.g. `style: add missing semicolons`)

### Subject

The subject contains a succinct description of the change:

* use the imperative, present tense: "change" not "changed" nor "changes"
* don't capitalize the first letter
* no dot (.) at the end

### Body

Just as in the **subject**, use the imperative, present tense: "change" not "changed" nor "changes".
The body should include the motivation for the change and contrast this with previous behavior.

### Footer

The footer should contain any information about **Breaking Changes** and is also the place to
reference GitHub issues that this commit **Closes**.

**Breaking Changes** should start with the word `BREAKING CHANGE:` with a space or two newlines. The
rest of the commit message is then used for this.

[conventional-commits]: https://www.conventionalcommits.org/en/v1.0.0/
[github-issue-closing]: https://help.github.com/articles/closing-issues-via-commit-messages/
