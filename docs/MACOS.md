# Native macOS preview

This port retains the upstream Tauri app and Rust data readers. It builds an
Apple Silicon `.app` and `.dmg` on macOS 15 or later.

## Available

- Native Steam installation detection, including additional Steam libraries.
- Select the Steam title directory, `CrimsonDesert_Steam.app`, or its
  `Contents/Resources/packages` directory.
- Save-directory discovery under `~/Library/Application Support/Pearl Abyss/CD/save`.
- Item browser, search, descriptions, icons, and crafting planner.
- Experimental native item spawning on the pinned Steam executable: launch from
  Item picker with a hash-verified save backup, then submit an explicit request.
  Receipts verify the inventory delta and prevent automatic retries.
  See [native inventory work](MACOS_ITEMS.md) for limits and live verification.
- Settings and local caches; installed apps use
  `~/Library/Application Support/CrimsonWorkbench`.

The catalog admits only exact matches for the checked-in table and localization
hashes. Every base table and the selected item localization are verified before
semantic parsing; crafting and icon tables have their own existing hash checks.
The Mac data reference is pinned to the checked native game build. A matching
catalog does not certify a vanilla installation, and the Mac schema has its own
cache identity. Future game updates may be rejected.

## Unsupported

Game-file application/restoration, save edits other than mount registration, runtime marketplace modules,
Extra Sockets, teleport, photo controls, and live world controls.
The existing process and executable guards continue to reject these operations
on macOS. Some upstream controls remain visible but cannot perform these actions.
Mod previews and exports are development tools; they have not been validated for
application to the Mac game. Normal catalog use does not change the game bundle,
signature, installed mods, or saves. Explicit inventory grants can be autosaved
by the game. The first one-Timber experiment passed independent UI confirmation
and save/reload verification; other item types have not been verified live.

## Install the preview

Download the DMG from the repository's [latest release](https://github.com/mpenfold1987/crimson-workbench-macos/releases/latest).
Open the DMG, drag Crimson Workbench to Applications, then Control-click the app
and choose **Open** the first time if macOS shows an unidentified-developer
warning. The preview is ad-hoc signed and is not notarized.

## Build from source

Install Apple command-line developer tools, Node 22.22+ and Rust 1.95+.
From the repository root:

```sh
cd app
npm ci
npm run desktop:mac
```

The outputs are under `target/release/bundle/macos/` and
`target/release/bundle/dmg/`. Local builds are not Developer ID signed or
notarized. See [Tauri's macOS packaging documentation](https://tauri.app/distribute/macos-application-bundle/).

## Verification

```sh
cargo test --workspace --locked
cd app
npm test
npm run test:e2e -- tests/setup.spec.ts
```

An explicit read-only integration test opens the installed Mac archives and checks
the 6,816-item catalog and 1,108 usable crafting recipes. It writes a search cache
only in a temporary project:

```sh
CD_MAC_GAME_DIR="/path/to/CrimsonDesert_Steam.app/Contents/Resources/packages" \
  cargo test --locked -p cd-core --test macos_live -- --ignored --nocapture
```

The diagnostic example compares all 61 referenced base, advanced, crafting, icon,
and item-localization files without admitting a build or parsing game tables:

```sh
cargo run --locked -p cd-core --example inspect_mac_compatibility -- \
  "/path/to/CrimsonDesert_Steam.app/Contents/Resources/packages"
```

Legacy live tests requiring private platform fixtures remain ignored by default.
Their status is separate from the Mac integration test above.


## Mount registration

The Mounts screen can register mounts in a selected complete save.save/lobby.save
pair while the game is closed. Each operation validates the catalog schema and
save hashes, backs up both files, journals its candidate and receipt, and checks
the written pair. Retrying a pending request uses the same ID without granting
another mount. Interrupted writes can be recovered from that journal.

Keep the game closed until registration finishes. macOS file locks serialize
Workbench writers; they do not stop Steam from launching the game or prevent
Steam Cloud from writing. Changed save generations are rejected rather than
overwritten. Backups are recorded in the registration receipt. Base-entry mounts
still require in-game summoning and riding verification.


## Live mounts

Select a mount in the Mounts screen, then use “Register selected mount live”
once the live module is connected. If needed, save and close the game first,
then use “Launch game with live mounts”; this creates a verified save backup.
Load a save and move briefly to establish the active character.

The action adds one previously unowned mount through the native stable
transaction. Existing ownership is checked in the loaded character's stable.
Save normally afterward. The raptor has been confirmed in game; other species
and variants remain unverified. See MACOS_ITEMS.md for the pinned build and
recovery limits.
