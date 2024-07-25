# Introduction

> **Disclaimer:** This book aims to analyze the purposes and properties of Blockchain technologies from an engineering point of view, without so much as requiring a technical IT background. This book is for you if you are interested in understanding the formalities of Blockchain, even if you are a beginner. While some exercises will be given in the [Rust](https://www.rust-lang.org/) programming language, these can be skipped entirely.

By the end of this book, you should have an in-depth understanding of the techniques and processes that come together to create a Blockchain network and, if you followed the Rust exercises, have created one yourself.

...

A **blockchain** is essentially a *distributed system of consensus*. In simpler terms, you can view a blockchain as a network of machines, all coming together to agree on a decision. This is complicated, because different machines might have a different view of the network. Consider the following example:

## High-school marble problem

**Alice** (A), **Bob** (B) and **Christie** (C) are each a different machine in our blockchain network. Let's imagine they are all players in a high school game of marbles. A, B and C each own a unique set of marbles. These marbles can be created, traded, or even destroyed!

> - **A**, **B** and **C** are different players.
> - Each players owns a unique set of marbles.
> - Marbles can be created, traded or even destroyed.

## Decentralized marble game

In the real (physical) world, it would be trivial to know how much marbles each individual has, and which marbles these are: you would only have to count them. However, in the virtual cypherspace of cryptocurrencies, this isn't as easy: there isn't a centralized point where we can view all of the marbles. Similarly, we can't rely on a single individual to tell us the truth: what if they lied!

> Blockchains resolve this problem in a variety of ways by allowing us to ensure, with either absolute certainty or a very high probability of certainty, that each player is telling the truth.

In the following chapters, we will be discussing practical solutions to the problems posed in this decentralized marble game, namely:

- How to ensure each player is telling the truth.
- How to punish bad behavior.
- How to safeguard the logical continuation of the network (here, the game).

But first, I would like to start with a brief history of the web and why these kinds of decentralized technologies are even needed in the first place.
