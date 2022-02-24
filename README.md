<p align="center">
  <img src="./assets/logo.png" width="80">
</p>
<h3 align="center">Kadium</h3>
<p align="center">
  App for staying ontop of YouTube channels' uploads
  <br/>
  <a href="https://github.com/probablykasper/kadium/releases"><b>Download for Mac, Windows or Linux</b></a>
</p>
<br/>

![Screenshot 1](assets/screenshot-1.png)
![Screenshot 2](assets/screenshot-2.png)

## Dev instructions

### Get started

1. Install Node.js
2. Install Rust
3. Follow the [Tauri setup guide](https://tauri.studio/en/docs/getting-started/intro)
4. Run `npm install`

### Commands

- `DEVELOPMENT=1 npm run dev`: Start app in dev mode. `DEVELOPMENT=1` tells it to use `./src-tauri/appdata` for app data.
- `npm run build`: Build
- `npm run lint`: Lint
- `npm run format`: Format

### Release new version
1. Update `CHANGELOG.md`
2. Bump the version number in `src-tauri/Cargo.toml`
3. Run `npm run check` to update `Cargo.lock`
4. Create a git tag in the format `v#.#.#`
5. Add release notes to the generated GitHub release and publish it
