# Changelog

All notable changes to RazeClicker are documented here.

## [2.3.1] - 2026-11-22

### Added
- Added the ability to enable and disable the Recorder.

### Improved
- Redesigned the About page.
- Improved the overall application layout and window size.
- Refined the application header and window controls.
- Reworked Ambient Effects using shader-based rendering.

### Fixed
- Fixed various minor bugs and issues.

---

## [2.3.0] - 2026-11-21

### Added
- Added Ambient Effects — subtle background lighting, atmospheric effects, and visual elements that give the interface more depth.
- Added Card Motion — interactive cards now subtly move away from the cursor, creating a more dynamic and responsive interface.
- Added smooth page transition animations for a more fluid navigation experience.

### Improved
- Completely redesigned the overall visual style of RazeClicker with a new color palette, updated styling, and a more modern appearance.
- Adapted all application pages to the new design while preserving their existing functionality.
- Redesigned the navigation layout, moving the navigation bar from the sidebar to the bottom of the window.
- Improved the layout and positioning of interface elements across all pages.
- Refined card styling, including backgrounds, borders, rounded corners, and spacing.
- Improved button, toggle, and selection control styling to match the new visual identity.
- Enhanced the overall visual consistency and user experience throughout the application.

### Changed
- Redesigned the Update dialog with an updated appearance and layout.
- Redesigned the Changelog dialog to match the new application design.


---


## [2.2.0] - 2026-11-14

### Added
- Added Jitter functionality to introduce random intervals between clicks.
- Added a Click Limit option to limit the number of clicks.
- Added a new Advanced page for customizing clicker behavior.
- Added a bug report button to the header for quick and easy bug reporting.
- Added an Always on Top option.
- Added an Overlay option.
- Added a toggle to enable or disable the clicker.
- Added more UI animations for a smoother and more dynamic user experience.

### Improved
- Improved the overall UI design for a cleaner and more polished appearance.

---

## [2.1.0] - 2026-11-12

### Added
- Added a new changelog dialog to display release notes after updating.

---

## [2.0.3] - 2026-11-12

### Fixed
- Fixed high CPU usage during clicking.

---

## [2.0.2] - 2026-11-11

### Fixed
- Fixed an issue where the previous version was not removed after updating.

---

## [2.0.1] - 2026-11-11

### Added
- Added a Reset All Settings option to restore default settings.

### Fixed
- Minor fixes and improvements.

---

## [2.0.0] - 2026-11-08

### Major Changes
- Completely rewritten RazeClicker from C++ to Rust.
- Introduced a new application architecture with a redesigned internal codebase.
- Replaced the old single-window layout with a new tab-based interface.

### Improved
- Redesigned and reorganized the Clicker interface.
- Moved location management into a dedicated Pick Location page.
- Redesigned the Recorder interface with dedicated hotkey configuration.
- Introduced a dedicated Settings page with centralized application preferences.
- Improved hotkey configuration and organization.

### Added
- Added language selection.
- Added an option to launch RazeClicker automatically when Windows starts.
- Added a dedicated page for application preferences and global hotkeys.

---

## [1.4.5] - 2026-10-28

### Added
- Added a Changelogs button to update notifications, allowing users to review changes before updating.

### Fixed
- Fixed the CPS field accepting letters. It now only accepts numeric input.

---

## [1.4.4] - 2026-10-27

### Added
- Added a Feedback button for quickly accessing the feedback page and submitting suggestions or bug reports.

---

## [1.4.3] - 2026-10-26

### Fixed
- Fixed an extra click occurring after stopping the clicker at low speeds.
- Stopping the clicker now immediately cancels the pending wait, preventing unwanted additional clicks.

---

## [1.4.2] - 2026-10-23

### Improved
- Improved the license confirmation interface with a cleaner design and better layout.

---

## [1.4.1] - 2026-10-22

### Added
- Added mouse button binding as a hotkey option in the keyboard clicker section.

### Fixed
- Fixed the positioning of the Hotkey text in the keyboard clicker section.

---

## [1.4.0] - 2026-10-22

### Added
- Added update notifications with automatic GitHub version checking.
- Added automatic updates, allowing users to download and install new versions directly from the application.

### Improved
- Updated the interface with a cleaner layout, refined colors, and improved controls.

---

## [1.3.1] - 2025-10-21

### Improved
- Improved macro recorder timing precision.
- Delays between actions and key or mouse button hold durations are now recorded and played back accurately.

---

## [1.3.0] - 2025-10-21

### Improved
- Reworked the click timing controller for more accurate CPS.
- Fixed timing drift and inconsistent clicking intervals.
- Improved keyboard clicker compatibility with games and applications.
- Improved stability and accuracy of keyboard input timing.

---

## [1.2.1] - 2025-10-20

### Maintenance
- Updated README documentation.

---

## [1.2.0]

### Maintenance
- Updated README documentation.

---

## [1.1.2] - 2025-07-21

### Fixed
- Fixed the misaligned icon on the window pin button.
- Fixed an issue where hotkey selection mode would not exit after assigning a hotkey.
- Fixed subsequent key presses changing the assigned hotkey after selection.

---

## [1.1.1] - 2025-05-09

### Fixed
- Fixed various bugs.

---

## [1.1.0] - 2025-05-07

### Added
- Added infinite playback for recorded actions.

### Improved
- Reduced window stuttering.
- Reduced RAM consumption.

---

## [1.0.0] - 2025-05-06

### Initial Release
- Added Toggle and Hold activation modes.
- Added left and right mouse button support.
- Added precise CPS control up to 500 clicks per second.
- Added multi-location clicking with user-defined coordinates.
- Added Always on Top functionality.
- Added customizable hotkeys (F1–F12).
- Added Keyboard Auto Clicker.
- Added action recording and playback.
