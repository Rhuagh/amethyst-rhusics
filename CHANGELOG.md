## Changelog

### v0.5.2
- Now linked to rustgd/rhusics instead of
   trsoluti/rhusics.
   
   Note: rhusics crate is still out-of-date in
   crates.io, so have to use GitHub source.
   
### v0.5.1
- Updated to Amethyst 0.15.3
- Added an example showing the system reporting
   collision events through Amethyst's collision
   system.

### v0.5.0
- Updated to latest Rhusics (0.8)
    - for now, linked to my fork (trsoluti).
- Updated to the latest Amethyst crate (0.15).
- Cleaned up examples to make them closer to
  the modern Amethyst style.
- Updated the README to link to Discord
  instead of Gitter.
- Also indicated this package will eventually
  be subsumed into Amethyst-Physics.
- Added this change log 
- BREAKING CHANGE
    - The size of boxes in the emitter is set
      to be the same as the bounding box size
      set when you bundle the emitter system.
      Previously the size was hard-coded.
- BREAKING CHANGE (dev only):
    - You need to modify the Cargo.toml file
      to run the examples on something other
      than MacOS. This is because Cargo
      does not yet support features that
      only affect development (Cargo bug #6915).
