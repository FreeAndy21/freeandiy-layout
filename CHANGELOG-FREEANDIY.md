<!-- Modified-by: FreeANDIY (Free Andy Agency), 2026. New file, not part of upstream DesignCraft. -->
# FreeANDIY Layout: changes from upstream DesignCraft

This file lists what the FreeANDIY fork changes in DesignCraft (Apache-2.0 section 4(b)). The
exact changes are generated from `freeandiy/` (see `NOTICE`); `git diff upstream-main main`
shows every changed file.

## Unreleased

### Branding
- Product name **FreeANDIY Layout** everywhere users see it: window title, menus, About, the start
  screen, messages, translations, installers, desktop entries, the web page, the CLI and MCP.
- Application id `agency.freeandy.layout` (was `ai.storyteller.designcraft`), so FreeANDIY builds
  install next to DesignCraft.
- Executables `freeandiy-layout` and `freeandiy-layout-cli`; the cargo packages keep their names.
- New app icon set and in-app mark (the angel Andy, Free Andy Agency) and DMG background.
- The UI brightness levels get FreeANDIY's flat print look (square corners, no shadows, the
  stamp-rose accent); Dark is ink and paper, Light paper and ink; High Contrast is unchanged.
- UI fonts IBM Plex Sans and IBM Plex Mono (OFL 1.1) on the desktop.

### Removed
- The ArtCraft name, wordmark and mark (`docs/brand/`, About, the start screen), as their terms
  require of forks.
- The ArtCraft Discord buttons and links (title bar, start screen, About, Help menu, macOS app
  menu, CLI, MCP, `app.links`).
- Upstream's release and platform workflows.

### Changed
- About lists the other FreeANDIY apps instead of ArtCraft's; links go to the FreeANDIY website
  and this repository.

### Added
- About: attribution to upstream DesignCraft and the Free Andy Agency seal.
- Slovak user interface (menus and common labels; the rest stays English for now).
- `NOTICE` section and `ASSETS.md` rows for FreeANDIY's assets and fonts; packages now ship
  `NOTICE`, `ASSETS.md`, the font licences and the brand terms.
