# Changelog

## Next
- Load 100 videos at a time instead of 50
- Allow selecting bottom row diagonally with the down arrow key

## 1.5.0 - 2023 Nov 19
- Add support for all channel URLs. URLs like `/lacunarecs`, `/@lacunarecs`, `/c/lacunarecs`, `/user/lacunarecs` and even `/lacunarecs/playlists` etc work now.
- Fix some errors not being shown
- Fix notification identifiers
- Scroll to bottom after adding channel

## 1.4.4 - 2023 Apr 3
- Fix blank screen
- Fix grid overflow

## 1.4.3 - 2023 Feb 2
- Fix new videos sometimes not loading when using arrow keys

## 1.4.2 - 2022 Nov 20
- Update Tauri

## 1.4.1 - 2022 Jul 8
- Fix "missing field `videoPublishedAt`" error (which happens when videos are privated but still accessible)

## 1.4.0 - 2022 Jun 28
- Fancy macOS single-color title bar
- Use Tauri 1.0, hopefully fixing some issues

## 1.3.2 - 2022 Jun 20
- Fix Content Security Policy issue

## 1.3.1 - 2022 Jun 20
- Fix archive button having an outline when clicked

## 1.3.0 - 2022 Jun 20
- Add `Escape` shortcut to reset focus
- Show loading indicator while checking for new videos
- Bring back focus when closing a modal
- Show tags alphabetically
- Use custom link drag image
- Keep selection and scroll position when archiving and unarchiving
- Use `actualStartTime` as the publish date for livestreams
- Fix new videos not showing up immediately
- Prevent archive/unarchive shortcuts when there is no selection
- Improve tab navigation
- Fix switching between New/Archived/All using arrow keys

## 1.2.3 - 2022 Mar 12
- Prevent double clicking the archive button from opening video
- Fix "0 of ?" showing on empty pages

## 1.2.2 - 2022 Feb 28
- Fix total video count not showing

## 1.2.1 - 2022 Feb 26
- Fix video loading issue

## 1.2.0 - 2022 Feb 26
- Clicking on videos now selects them. To open a video, double click or cmd/ctrl+click on it.
- Add `Cmd/Ctrl+Delete` shortcut to archive the selected video
- Add `Shift+Cmd/Ctrl+Delete` shortcut to unarchive the selected video
- Add `Cmd/Ctrl+,` shortcut to open the settings
- Small design changes
- Fix incorrect video page loading
- Fix missing Windows release
- Fix channel title line wrapping

## 1.1.0 - 2022 Feb 23
- Sort videos from new to old
- Increase size of video thumbnails
- Fix channel name sometimes appearing on the same line as the title
- Fix thumbnail sizing

## 1.0.2 - 2021 Nov 12
- Fix weekday being displayed as date

## 1.0.1 - 2021 Nov 12
- Fix Videos page constantly refreshing

## 1.0.0 - 2021 Nov 12
- Initial release
