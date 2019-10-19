# Version 0.4.0

- Add deprecation note ([PR #3](https://github.com/crossterm-rs/crossterm-utils/pull/3))
- Remove all references to the crossterm book ([PR #4](https://github.com/crossterm-rs/crossterm-utils/pull/4))
- Remove unsafe static mut ([PR #5](https://github.com/crossterm-rs/crossterm-utils/pull/5))
  - `sys::unix::RAW_MODE_ENABLED` replaced with `sys::unix::is_raw_mode_enabled()` (breaking)
  - New `lazy_static` dependency

# Version 0.3.1

- Maintenance release only
- Moved to a [separate repository](https://github.com/crossterm-rs/crossterm-utils)
