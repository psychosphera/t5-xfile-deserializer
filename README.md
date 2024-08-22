# t5-xfile-deserializer
## A tool to deserialize Fastfiles (internally called `XFiles`) for T5.

`XFiles` are the primary, proprietary storage format used by the IW engine, and thus, by T5. Windows and presumably macOS builds of BO1 use `IWDs` to store some assets, but `XFiles` are still utilized heavily, PS3 does the same thing but with `PAKs`, and Xbox 360 almost exclusively uses `XFiles` (it uses a couple `PAKs` too).

Data stored by `XFiles` includes shaders, the clipmap/PVS, scripts, animations, models, fonts, BSP data, menus, weapons, and more.

### What can currently be deserialized
1. Material Technique Sets
2. Materials
3. Images (textures)
4. PhysPresets
5. PhysConstraints
6. XAnims
7. XModels
8. LightDefs
9. String Tables
10. RawFiles

### What can probably be deserialzed (untested)
1. XGlobals
2. Map Ents
3. Localize Entries

### What will soon be able to be deserialized (implemented but bugged)
1. Destructibles
2. Fonts
3. Impact Effects
4. Effects
5. Gameworlds (SP & MP)
6. Sounds
7. Sound Patches
8. SndDriverGlobals
9. DDLs
10. Glasses
11. Emblem Sets
12. Menu Lists
13. Menus
14. Weapon Variants
15. ComWorld
16. GfxWorld
17. Clipmap/PVS

## Building
```bash
    $ git clone https://github.com/Ashlyyn/t5-xfile-deserializer.git
    $ cd t5-xfile-deserializer
    $ cargo build
```

## Running
To run, you'll need to supply an `XFile` (which I naturally can't provide here). `XFiles` aren't backwards nor forwards compatible (see `lib.rs` for an explanation), so it'll have to be an XFile from BO1 specifically. The library will currently reject any that don't match the correct version. It *will* accept `XFiles` from non-Windows builds, albeit with a warning. 

I primarily created this to integrate into OpenT5 once it's done, but I figured it could be useful as a standalone project in case someone else has a use for it. Some of the structure definitions here are probably identical or very similar for, e.g., T4 or T6 (or even IW3), so this could probably serve as the groundwork for deserializing their `XFiles` (not something I plan on doing though).

## Todo
1. Fix deserialization logic of remaining `XAsset`s.
2. Get offsets working.
3. Account for shared pointers. (All pointers get boxed currently, but that's definitely not correct semantically for a lot of them.)
4. Relatedly, account for linked lists.
5. Tidy up the deserializer's API (typestated now, but still a little janky).
6. Better CLI for the binary.
7. Then debug them (yay...).
8. Make sure all the arrays sized by `MAX_LOCAL_CLIENTS` were caught (pretty sure a couple in `techset.rs` slipped through).
9. Verify whether macOS `XFiles` are identical to Windows.
10. Verify whether Wii even uses `XFiles`.
11. Docs (lol)

## Future Todo
1. Account for differences in macOS `XFiles`, if they're different from Windows.
2. Create D3D9 textures.

## Far Future Todo
1. Account for differences in console `XFiles`.
2. Serialization.