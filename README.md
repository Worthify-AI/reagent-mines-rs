# ReAgent Mines in Rust

A playable, bounded reimplementation of Simon Tatham’s **Mines**, with a classic
presentation reconstructed from executable screenshots. Our code is MIT licensed. We chose this permissively licensed
reference to demonstrate the method while respecting its creators.

## What we supplied and observed

The reconstruction used the x86-64 Linux executable from Ubuntu’s sgt-puzzles
package, version 20230410.71cf891-2build2, its license, public help and runtime
observations. Original Mines implementation source and existing Mines
reimplementation code were withheld during construction. Familiar mechanics
and possible model prior knowledge were not eliminated.

Reference ELF SHA-256:
94526720b3e0491b59a2d9cb02bfd353d374199753e55c996d7430d9344d1b67

Worthify ReAgent ingested the binary and completed a bounded metadata/string
triage. Worthify Decompiler produced native output for all 212 ingested functions,
with zero per-function failures or timeouts. Its adapter reported gaps in semantic
IR and source lineage. These counts describe output production, not correctness.
A human-directed coding agent collected reference interactions and wrote and
tested this independent Rust implementation. Broader ReAgent analyses encountered
completion-receipt failures. The newer Arena desktop provisioning path was not demonstrated. A later Neverball experiment exercised the legacy Docker/QEMU guest connection. This is not a claim of autonomous or universal recovery.

## Scope

Implemented: reveal, flag/unflag, adjacent mine counts, empty-region expansion,
chording, first-click safety, reset, counters, win/loss and ignored invalid or
terminal actions. The ordinary generator is our own; it excludes the first
square and its neighbors from mine placement.

Excluded: the reference’s no-guess generation algorithm, solver and complete save-file compatibility. **Reference** cycles eight observed fixed boards and displays their descriptive
Game IDs. It does not decode arbitrary IDs or use the reference RNG. Undo and
cumulative death-history behavior are outside this toy's supported scope.

The original comparison fixtures cover eight boards and 36 observed traces: 24 construction comparisons
and 12 initially withheld holdouts. The first holdout run caught a wrong-flag chord
discrepancy. We fixed it; those holdouts are now regression fixtures, not evidence
of untouched blind success. Final cell-state comparisons pass. The fixtures do
not measure timing or visual equivalence. Two later flagged-empty-region traces
cover the covered hole left after removing a safe flag and clicking an open zero.

## Play

- Left click / tap: reveal or chord an open number.
- Right click: flag or unflag.
- Arrow keys: move; Enter: reveal; Space: flag.
- F or the visible flag-mode button: toggle touch-friendly marking.
- R: restart; N: new board.
- Reference / B: cycle eight observed fixed boards at the reference's 20 pixel cell size.
- Flagged empty square / Q: step through the board-1 flag/flood/unflag/open-zero sequence.
- Original size / Touch size: resize the presentation without changing board state.

The canvas has keyboard controls and live status announcements. It does not
provide a complete nonvisual board interface.

## Build and check

Rust stable with the wasm32-unknown-unknown target. Cargo.lock pins dependencies.

~~~sh
cargo fmt --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
bash scripts/build-web.sh
python3 -m http.server 39720 --directory web
~~~

For repeatable browser checks, in a second terminal:

~~~sh
npm ci --ignore-scripts
npx playwright install chromium
npm run test:browser
~~~

MINES_URL selects another preview address; PLAYWRIGHT_CHROMIUM_EXECUTABLE selects
an installed isolated browser. The loader is self-hosted from miniquad 0.4.8.
web/loader-manifest.json records loader and included WASM hashes.
The .cargo/config.toml linker setting permits intentional WebGL/JS imports.

## Attribution

Reference: [Simon Tatham’s Portable Puzzle Collection](https://www.chiark.greenend.org.uk/~sgtatham/puzzles/).
See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) and licenses/ for the full
reference notice, dependency terms and embedded font notice. Board colors, cell bevels, bold number glyphs and icon geometry follow runtime
screenshots. The working Worthify controls sit outside the game panel; the
reference's menus are not reproduced. DejaVu Sans is bundled with its font license.
Font rasterization and icon edge antialiasing can differ from the native GTK
reference; this is not a claim of pixel-identical output. No original game binary, original implementation, private research,
ReAgent source, infrastructure trace or credentials are included.

A later live UI acceptance run at a 30-second per-function budget verified all 212 saved native outputs and provenance, plus a selected function rerun and revision reload. A separate run at the 5-second UI default produced 210 outputs and two timeouts. These are output and persistence checks; semantic IR remained unavailable. Later provider availability and quota problems interrupted chat. Catalog routing and the initial completion-tool schema were repaired in the isolated ReAgent environment. Fresh metadata and lexical-search tasks then completed through an existing configured provider. These are bounded static checks; embedding requests still failed.

## Additional reference quirks

A separate black-box run compared 97 within-scope action steps: 95 initially matched; the two open-zero discrepancies matched after correction. The original 36 traces/272 steps still pass. The extra observations include twelve flags on ten mines, blocked expansion and ignored under/over-flag chords. The reference displays `Marked:12/10`; our remaining-flags state is −2. These are observed quirks, not established original bugs.

Reference seeds were exercised only in the original executable. Two runs with the same seed and first click matched; changing the first click changed the layout. We make no cross-generator RNG claim. The original generated-game Restart restores the first revealed region; our independently generated-game Reset returns to a covered board. Undo and persistent death counts remain unsupported.
