# Crimson Workbench for macOS

Free, open-source tools for a local macOS installation of Crimson Desert. Search
the item catalog, plan crafting, manage available mods, and add supported items
to your open game inventory from one native Apple Silicon app.

This is an independent macOS port of
[ArasHueseyin/crimson-workbench](https://github.com/ArasHueseyin/crimson-workbench).
Thank you to [Aras Hueseyin](https://github.com/ArasHueseyin) and every
contributor to the original project for creating and open-sourcing its foundation.

[macOS usage and build notes](docs/MACOS.md) · [Contributing](CONTRIBUTING.md) · [Report a problem or request a mod](https://github.com/mpenfold1987/crimson-workbench-macos/issues/new/choose)

![Workbench's Item Picker adding ten masks to the open game inventory](docs/assets/item-picker-demo.gif)

Adding ten masks while the game is running. This recording shows an earlier Workbench interface.

## What you can do

- **Item Picker:** search localized items with icons and add supported items to your inventory while playing.
- **Crafting:** browse recipes and plan the materials you need.
- **Mod management:** browse the bundled marketplace and review each feature's availability.
- **Backups and recovery:** take a verified save backup before a live inventory request.

See the [Marketplace guide](docs/MARKETPLACE.md) for individual mods and limits, or the [documentation index](docs/README.md) for detailed guides.

## Install and compatibility

Download **Crimson Workbench.dmg** from the [latest macOS release](https://github.com/mpenfold1987/crimson-workbench-macos/releases/latest), open it, and drag the app to Applications. The preview build is ad-hoc signed, so macOS may require you to Control-click the app and choose **Open** the first time.

To build from source, follow the [macOS usage and build notes](docs/MACOS.md).
It targets Apple Silicon and detects a Steam installation of the game. The app
checks the exact supported native game build before enabling live inventory
requests, and creates a verified backup before it starts the game for one.

The interface is in English. Item names follow the installed Steam language when
detected, with English as the fallback and a manual language selector. After a
game update, live functionality may need an update too; check
[build support](docs/BUILD_SUPPORT.md) before using a different build.

## Contribute

Start with [CONTRIBUTING.md](CONTRIBUTING.md) for the PR workflow and [the developer guide](docs/DEVELOPMENT.md) for setup, a code map and test commands. Frontend and synthetic tests can run without owning the game.

Bug reports, feature requests, mod ideas and feedback are welcome through the [issue forms](https://github.com/mpenfold1987/crimson-workbench-macos/issues/new/choose).

## License and credits

This port and the original Workbench code are licensed under [MIT](LICENSE).
Dependencies retain their own licenses; see [credits](CREDITS.md) and
[third-party notices](THIRD_PARTY_NOTICES.md). Crimson Desert belongs to Pearl
Abyss; this is an independent project. Game archives, saves and personal
configurations are not distributed.
