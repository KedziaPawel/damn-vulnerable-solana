# 🕵️‍♂️ Damn Vulnerable Solana

A collection of **deviously vulnerable** Solana smart contract challenges designed to make you think, break, and (hopefully) learn something about Solana security.

Think you're **unstoppable**? Let’s see if you can crack these bad boys.

## 🏆 Challenges

1. [💸 Wallet What Wallet?](./challenges/wallet-what-wallet/description.md) – Can you withdraw what’s not yours? 🤔
2. More coming soon... because breaking things is fun!

## ⚙️ Prerequisites

Before you start hacking away, make sure you have:

- **Rust** – Because Solana contracts are written in Rust, not JavaScript. Tested with Rust stable version 1.85.1.

## 🔨 Build

Compile all challenges with:

```bash
cargo build
```

## 🎯 How to Solve Challenges

Each challenge has its own directory with:

- **`src/`** – The smart contract code (⚠️ Hands off! No modifying this!).
- **`tests/`** – The **only** place you're allowed to mess with. Your mission: break the contract in the `hack` function.
- **`description.md`** – Tells you what’s up.
- **`hint.md`** – A little nudge if you’re stuck.
- **`solution.md`** – Spoilers! Check this only if you give up.

### ⚔️ Running Your Attack

Test your exploit against a specific challenge with:

```bash
cargo test --test CHALLENGE_NAME
```

For example, to test **Wallet What Wallet?**, run:

```bash
cargo test --test wallet-what-wallet
```

If the test passes, congrats! You’ve successfully **hacked** it. 💀

## 🔥 Inspired by Solana CTF

Shoutout to [Solana CTF](https://github.com/neodyme-labs/solana-ctf) creators for the inspiration. If you enjoy breaking things, check them out too!

## ⚠️ Work in Progress: Expect Changes

This repo is under heavy development 🚧, and the folder structure might change as more challenges and features are added. Keep that in mind as you explore—things could get shuffled around! 🙃

## 🤝 Contributing

Want to make this even **more** fun? Help us by:

1. Creating **new** challenges
2. Making existing ones **even harder**
3. Fixing bugs (or adding more… but don’t tell anyone)
4. Improving documentation

## 📜 License

This project is under the **MIT License**, so feel free to fork, break, and share.
