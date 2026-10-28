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
and 50 as Black. It prints progress after every game, then White wins, draws,
Black wins, and each engine's wins, draws, and losses.

Edit `ENGINE_A` and `ENGINE_B` in `src/bin/engine_match.rs` to compare other
searchers, evaluators, or depths. Update each configuration's name along with its
factory. Both modes use these factories; the defaults are negamax with
`BoardEvaluation` at depths 2 and 5. Each benchmark game starts from the standard
position with fresh engines and empty search caches. Random tie-breaking is
preserved, so results can vary between runs.

Each run creates a uniquely named CSV under `benchmark-results/` in the current
directory. It records engine assignments, results (`white_win`, `draw`, or
`black_win`), ending reasons, plies, elapsed milliseconds, and cumulative colour
totals. The final row contains the overall White/draw/Black score. Results are
flushed after every completed game, and previous files are preserved.

Checkmate awards the win to the opposite colour from the side to move.
Stalemate, threefold repetition, and the fifty-move rule are draws. A game still
running after 400 plies (200 full moves) is recorded as a draw with the distinct
reason `move_limit`; natural endings on the final ply take precedence.

The benchmark feature skips UI startup and retains the existing SDL build
dependencies. It uses the current engine behaviour, including the pending
cache-key issue, to support before-and-after comparisons.

Run the tests for both modes:

```powershell
cargo test --all-targets
cargo test --all-targets --features benchmark
```

Ordinary tests do not run the full 100-game benchmark.
