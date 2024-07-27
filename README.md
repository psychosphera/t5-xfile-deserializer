# t5-xfile-deserializer
## A tool to deserialize Fastfiles (internally called `XFiles`) for T5.

`XFiles` are the primary, proprietary storage format used by the IW engine, and thus, by T5. Windows and presumably macOS builds of BO1 use `IWDs` to store some assets, but `XFiles` are still utilized heavily, PS3 does the same thing but with `PAKs`, and Xbox 360 almost exclusively uses `XFiles` (it uses a couple `PAKs` too).

Data stored by `XFiles` includes shaders, the clipmap/PVS, scripts, animations, models, fonts, BSP data, menus, weapons, and more.

### What can currently be deserialized
1. Material Technique Sets
2. Materials
3. Images (textures)

### What can probably be deserialzed (untested)
1. String Tables
2. XGlobals
3. Raw Files
4. Map Ents
5. Localize Entries

### What will soon be able to be deserialized (implemented but bugged)
1. Destructibles
2. Fonts
3. Impact Effects
4. Effects
5. Gameworlds (SP & MP)
6. Lights
7. XAnims (animations)
8. XModels (models)
9. Phys Constraints
10. Sounds
11. Sound Patches
12. Sound Driver Globals
13. DDLs
14. Glasses
15. Emblem Sets
16. Menu Lists
17. Menus
18. Weapon Variants
19. ComWorld

### What hasn't yet been implemented
1. Clipmap/PVS
2. GfxWorld

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
1. Fix deserialization of `XModel` and whatever deserialization problems come next. (Probably a lot.)
2. Get offsets working.
3. Eliminate global state (probably going to require adding the deserializer as a parameter of `XFileInto`, which will involve changing every single `impl` and call. Not fun.)
4. Account for shared pointers. (All pointers get boxed currently, but that's definitely not correct semantically for a lot of them.)
5. Relatedly, account for linked lists.
6. Tidy up the deserializer's API.
7. Better CLI for the binary.
8. Implement the remaining unimplemented `XAssets` (Only two left!).
9. Then debug them (yay...).
10. Verify whether macOS `XFiles` are identical to Windows.
11. Verify whether Wii even uses `XFiles`.

## Future Todo
1. Account for arrays whose size depends on `MAX_LOCAL_CLIENTS`. Currently `MAX_LOCAL_CLIENTS` is just hard-coded to `1`, which is correct for Windows and presumably macOS, but certainly isn't for consoles. This will probably involve using a const generic to size the arrays, which will end up propogating all the way up every `XAsset` `struct` that contains one or points to a `struct` that contains one. Again, not fun.
2. Account for differences in macOS `XFiles`, if they're different from Windows.

## Far Future Todo
1. Account for differences in console `XFiles`.