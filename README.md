# MiSide Secret Menu

Mod for MiSide that makes secret alternative menu available all the time.

## Installation

1. Download the latest version of the mod from the [releases page](https://github.com/feeeek/miside-secret-menu-unlocker/releases).
2. Download `BepInEx-Unity.IL2CPP-win-x64-6.0.0-pre.2.zip` from the [BepInEx releases page](https://github.com/BepInEx/BepInEx/releases/tag/v6.0.0-pre.2).
3. Extract the contents of `BepInEx-Unity.IL2CPP-win-x64-6.0.0-pre.2.zip` into the game's root folder (where `MiSide.exe` is located).
4. Extract `SecretMenu` folder from the mod archive into the `BepInEx/plugins` folder.
5. Launch the game.

## Showcase

![Screenshot](showcase/SecretMenu.png)

## Building

1. Install dotnet SDK 6.0 or newer
2. Install rust and cargo
3. Run `cargo build --release`
4. You should get `target/release/secretmenu_native.dll` and `bin/Release/net6.0/SecretMenu.dll` files

## Contributing

If you want to contribute, please, make sure you know what you are doing.
I did this for fun as a proof of concept rust mod for Unity, this is not something I plan to maintain or support.
