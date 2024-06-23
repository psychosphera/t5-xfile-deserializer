# t5-xfile-deserializer
## A tool to deserialize Fastfiles (internally called `XFiles`) for T5.

`XFiles` essentially contain level data for the engine's maps. Assets (textures, sounds, etc.) tend to be stored in `IWD`s instead so that they can be referenced by multiple maps. However, `XFiles` *can* store those too.

Data stored by `XFiles` includes shaders, the clipmap, the PVS, scripts, animations, models, fonts, BSP data, menus, weapons, and more.

### What can currently be deserialized
1. Materials
2. Material Technique Sets

### What can probably be deserialzed (untested)
1. String Tables
2. XGlobals
3. Raw Files
4. Map Ents
5. Localize Entries

### What will soon be able to be deserialized (implemented but bugged)
1. Destructibles
2. Fonts
3. Effects
4. Impact Effects
4. Gameworlds
5. Lights
6. XAnims (animations)
7. XModels (models)
8. Phys Constraints
9. Images (textures)

### What hasn't yet been implemented
1. Sounds
2. Sound Patches
3. Clipmap
4. Clipmap PVS
5. ComWorld
6. GfxWorld
7. UI Maps
8. Menu Lists
9. Menus
10. Weapons
11. Weapon Variants
12. Sound Driver Globals
13. DDLs
14. Glasses
15. Emblem Sets

## Building
Building requires a nightly toolchain.
```bash
    $ git clone https://github.com/Ashlyyn/OpenT5.git
    $ cd OpenT5
    $ cargo +nightly build
```

## Running
To run, you'll need to supply an `XFile` (which I naturally can't provide here). `XFile`s aren't backwards nor forwards compatible (see `lib.rs` for an explanation), so it'll have to be an XFile from the most recent build of BO1 PC specifically. Earlier builds and builds for other platforms may or may not work, but I'm not going to test them, and the program will currently reject any that don't match the correct version. 

I primarily created this to integrate into OpenT5 once it's done, but I figured it could be useful as a standalone project in case someone else has a use for it. Some of the structure definitions here are probably identical or very similar for, e.g., T4 or T6 (or even IW3), so this could probably serve as the groundwork for deserializing their `XFile`s (not something I plan on doing though).
