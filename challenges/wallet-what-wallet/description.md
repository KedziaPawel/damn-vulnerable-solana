**💰 Wallet What Wallet? 💰**

A simple personal wallet contract—deposit SOL, withdraw SOL. Easy, right?

Each wallet has **one** authority (stored in the account struct):

```rust
pub struct Wallet {
    pub authority: Pubkey
}
```

Only the authority can withdraw funds... **or can they?** 🧐

Think you can outsmart the system? **Hack it.** 😏
