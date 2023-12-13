# Contributing

## Introduction

This project is about Smart Intermediate Representation, a brand-new smart contract compiler framework on multi-chain, 
and other explanations. 

If you want to be part of this effort here are some ways you can participate:

## Discussion board

If you have a question or an idea regarding certain content, but you want to
have feedback of fellow community members, and you think it may not be
appropriate to file an issue open a discussion in our
[discussion board]().

## Writing a new feature

Before writing a new feature please check in one of the following resources if
there is an existing discussion or if someone is already working on that topic:

- [All issues](https://github.com/AntChainOpenLabs/Smart-Intermediate-Representation/issues),
- [Pull Requests](https://github.com/AntChainOpenLabs/Smart-Intermediate-Representation/pulls)

If you don't find an issue regarding your topic, and you are sure it is not more
feasible to open a thread in the
[discussion board]()
please open a new issue, so we can discuss the ideas and future content of the
article together and maybe give some feedback/input on it.

Consider developing your feature in a way that has a low barrier of entry so also
[Rustlings](https://github.com/rust-lang/rustlings) can follow and understand
the thought process behind it. So we can encourage people to use these patterns
early on.

## Style guide

In order to have a consistent style across the book, we suggest to:

- Follow the official Rust Style Guide's
  [style guide](https://doc.rust-lang.org/nightly/style-guide/).
- Follow
  [RFC 1574](https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md#appendix-a-full-conventions-text).
  Tl;dr:
  - Prefer full types name. For example `Option<T>` instead of `Option`.
  - Prefer line comments (`//`) over block comments (`/* */`) where applicable.

## Check the code locally

Before submitting the PR launch the commands `make` to make sure that
the code builds and `make test` to make sure that tests are correct.

### Cargo lint

To make sure the files comply with our Rust style we use
[Clippy](https://github.com/rust-lang/rust-clippy). To spare
you some manual work to get through the CI test you can use the following
commands to automatically fix most of the emerging problems when writing
Rust files.


- Check all rust files:
  - unix: `make fmt-check`
  - windows: ``

- Automatically fix basic errors:
  - unix: `make fmt`
  - windows: ``

## Creating a Pull Request

"Release early and often!" also applies to pull requests!

Once your article has some visible work, create a `[WIP]` draft pull request and
give it a description of what you did or want to do. Early reviews of the
community are not meant as an offense but to give feedback.

A good principle: "Work together, share ideas, teach others."

### Sign the CLA

Since the first time you push a PR request as a contributor, you will be asked to sign a CLA (Contributor License Agreement), please following the instruction to comment in such PR.

### Important Note

Please **don't force push** commits in your branch, in order to keep commit
history and make it easier for us to see changes between reviews.

Make sure to `Allow edits of maintainers` (under the text box) in the PR so
people can actually collaborate on things or fix smaller issues themselves.