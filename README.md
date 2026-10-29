# Chess engine

Run a visual match between two AI configurations:

```powershell
cargo run --release --bin engine_match
```

Run a batch of 100 games without opening a window:

```powershell
cargo run --release --bin engine_match --features benchmark
```

The benchmark alternates colours each game, so each engine plays 50 games as White
and 50 as Black. It prints each engine's running win count and shared draws after
every game, then each engine's total wins, draws, and losses.

Edit `ENGINE_A` and `ENGINE_B` in `src/bin/engine_match.rs` to compare other
searchers, evaluators, or depths. Update each configuration's name along with its
factory. Both modes use these factories; the defaults are negamax with
`BoardEvaluation` at depths 2 and 5. Each benchmark game starts from the standard
position with fresh engines and empty search caches. Random tie-breaking is
preserved, so results can vary between runs.

Each run creates a uniquely named CSV under `benchmark-results/` in the current
directory. It records colour assignments, the winning engine's name (or `draw`),
ending reasons, plies, elapsed milliseconds, and cumulative `engine_a_wins`,
`draws`, and `engine_b_wins`. Engine A and B retain their identities when colours
swap. The final row contains the overall engine score. Results are
flushed after every completed game, and previous files are preserved.

Checkmate awards the win to the opposite colour from the side to move.
Stalemate, threefold repetition, and the fifty-move rule are draws. A game still
running after 400 plies (200 full moves) is recorded as a draw with the distinct
reason `move_limit`; natural endings on the final ply take precedence.

The benchmark feature skips UI startup and retains the existing SDL build
dependencies.

Run the tests for both modes:

```powershell
cargo test --all-targets
cargo test --all-targets --features benchmark
```

Ordinary tests do not run the full 100-game benchmark.
