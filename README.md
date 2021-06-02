# Emotes Pallet

A pallet allowing users to emote on entities in the Substrate ecosystem

## Functionality

Users can send emotes to any on-chain entity (AccountID, NFT ID, any other asset). Originally implemented as remarks via https://github.com/rmrk-team/rmrk-spec/blob/master/standards/rmrk1.0.0/interactions/emote.md

An emote is a switch. Sending a 🚀 twice from the same accounts results in that rocket no longer being applied to that entity.

## Namespaces

An entity being emoted on should be namespaced, so it is clear which group of entities it belongs to (i.e. they may be an NFT with an ID that matches an account ID - which entity is being emoted on?)

Current namespaces:

|Namespace|References|
|----|----|
|pubkey|Public key of an account, allows emoting on people's accounts|
|rmrk1|ID of RMRK 1.0 NFT|
|rmrk2|ID of RMRK 2.0 NFT|
|ink|Unique identifier of smart contract|
|eth|Hex of smart contract or Ethereum EOA|

## Supported Emote set

Any UTF code matching an emote should be supported, even multi-skin ones. These should be hard-coded to prevent people from sending arbitrary messages. For composable emoji (like skin modifiers) each modifier should be checked separately (they are usually represented like so: `\U0001f44b\U0001f3fe`, or with any separator between the unicodes).

## Challenge

The challenge is developing a cheap but performant implementation which allows looking at emotes over-time. This is necessary to develop good notification systems that can check for new emotes on owned property (which of my NFTs got upvotes? Did my account get an emote?) but also to develop a good timeline of interactions (Gavin upvotes and NFT on Tuesday and unvotes it on Wednesday is an important piece of metadata people might value).

Therefore, querying by namespace and entity ID is of paramount importance, but optimizations should be put in place that allow mass-queries per many IDs, for use cases where accounts own many NFTs and need to check the state of them all. A separate data structure holding aggregate information for fast-fetching is not out of the question.

This pallet needs to be benchmarked for complexity at high load both incoming (many incoming transactions) and persisted (queries of big sets of data).

## ED and refcount

Generally when an account has some interactivity on-chain, it needs to *exist*, and the existence is reserved via an [existential deposit](https://wiki.polkadot.network/docs/en/build-protocol-info#existential-deposit) or via increasing / decreasing refcount. With emotes, we do not think this is necessary if the data is optimized enough. However, the problem of old data and account pruning remains a cryptoeconomic problem that's worth exploring:

- do we wipe all emotes by an account when this account is reaped?
- do we require an account to make a deposit of some kind to gain emote functionality, treating it similarly to the Identity pallet?
- do we *squash* emotes older than X period of time to conserve space, and only monitor granular time-series data within the X period? It is arguably unnecessary to have total details about emotes on a 2 year old dormant NFT, and someone wanting these details can sync with an archive node instead and replay the extrinsics.

The above problems are up for discussion and analysis.

## Custom Functionality

The pallet should also allow the root origin (Council or Sudo) to:

- change the emote cost
- blacklist a user
- wipe all emotes by a certain user
