# Spotifast

This is [31780’s fork](https://github.com/31780/spotifast) of
[Carmine Paolino’s Spotifast](https://github.com/crmne/spotifast), with additional
security fixes. Download this fork from its [Releases page](https://github.com/31780/spotifast/releases).

**Spotify, native and fast.** Spotifast is a Spotify client written in
Rust with [egui](https://github.com/emilk/egui). It plays music through
[librespot](https://github.com/librespot-org/librespot), typically uses
100–250 MB of RAM, starts in well under a second, and has no browser engine.
It runs on Linux, macOS, and Windows.

**Playback needs Spotify Premium.** Free accounts can browse and search, but
cannot play music through Spotifast.

https://github.com/user-attachments/assets/a5f669ce-b3b7-4f8e-9933-976a78876c7e

![Spotifast Home with the playlist library, recommendations, queue, and player visible](docs/screenshot.png)

**[spotifast.rocks](https://spotifast.rocks/)** is the upstream project’s guide.
Its downloads and package-manager commands install upstream builds:

- [Getting started](https://spotifast.rocks/getting-started/): sign-in, playback on this computer, themes, fonts, proxies
- [Everyday use](https://spotifast.rocks/using-spotifast/): keyboard shortcuts, command-line control, updates
- [Settings and files](https://spotifast.rocks/settings-and-files/) and [Privacy](https://spotifast.rocks/privacy/)
- [How it connects](https://spotifast.rocks/how-it-connects/) and [What Spotify allows](https://spotifast.rocks/what-spotify-allows/)
- [Will my account get banned?](https://spotifast.rocks/what-is-spotifast/#will-my-spotify-account-get-banned)

**Want WhatsApp just as fast and native?** [ZapFast](https://zapfast.rocks)
is Spotifast's sibling. Both are built on
[fastframe](https://github.com/crmne/fastframe).

## Install this fork

Open [Releases](https://github.com/31780/spotifast/releases) and select a release:

- **Windows:** download the `x86_64-pc-windows-msvc-setup.exe` installer for most
  PCs, or `aarch64-pc-windows-msvc-setup.exe` for Windows on ARM. A portable ZIP
  is also available. The ARM build does not include MilkDrop.
- **macOS:** download `macos-universal.dmg`, open it, and drag Spotifast into
  Applications. It supports both Apple Silicon and Intel Macs.
- **Linux:** download `x86_64-unknown-linux-gnu.tar.gz`, or
  `aarch64-unknown-linux-gnu.tar.gz` for ARM64. Extract it and open `spotifast`.
  Keep `spotifast-portable.txt` beside the executable. These builds require
  glibc 2.39 or newer, such as Ubuntu 24.04 or Debian 13, and desktop audio and
  graphics libraries. See [Linux dependencies](https://github.com/31780/spotifast/blob/main/PACKAGING.md).

These test builds are not Apple-notarized or Windows Authenticode-signed.
Windows or macOS may warn about an unidentified publisher. Download only from
this fork’s Releases page and review the warning before deciding whether to run
it. The signed checksums protect in-app updates; they do not replace platform
code signing.

Quit any existing Spotifast before installing. This fork uses the same app
identity and settings as upstream, so it replaces an existing installation.

### Spotify login

Sign in with your own Spotify account, then enable playback on this computer
and approve its separate browser sign-in. Local playback requires Premium.
The downloads contain no Spotify accounts, saved sessions, or personal Client ID.

If login works but loading is slow, follow
[personal app setup](https://github.com/31780/spotifast/blob/main/docs/_guide/make-it-even-faster.md) in Settings → Account.
Use your own Spotify developer app and its Client ID, never its Client Secret.
Its redirect URI must be exactly `http://127.0.0.1:8989/login`.

**Known limitation:** first-time login still uses a shared Spotify app, which
can be rate-limited. If browser approval succeeds but Spotifast keeps waiting,
wait before trying again. Personal app setup currently requires reaching
Settings after sign-in; this release does not yet provide that setup on the
login screen. Report persistent trouble in this fork’s issues without posting
callback URLs, tokens, or logs containing credentials.

Prereleases are test downloads and are not offered by automatic update checks.
Get subsequent test builds from Releases. Stable in-app updates use this fork’s
repository and signing key.

## Contributing

This checkout includes security hardening for signed updates, playback log
redaction, and bounded browser sign-in callbacks. See
[How it connects](docs/_reference/how-it-connects.md) for the details.
Updates use this fork’s release feed and require its publisher signature.
Automatic update downloads remain off by default.

Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening an issue or pull
request. To look at the interface without a Spotify account, run
`cargo run --features demo -- --demo`. Translations live in `assets/i18n/`;
see [Translating Spotifast](docs/_reference/translating.md). Release
packaging is described in [PACKAGING.md](PACKAGING.md).

## Acknowledgements

Spotifast uses [librespot](https://github.com/librespot-org/librespot),
[egui](https://github.com/emilk/egui), the [Inter](https://rsms.me/inter/)
typeface (OFL), and [Lucide](https://lucide.dev) icons (ISC).

Spotifast is an independent project and is not affiliated with Spotify.
Spotify is a trademark of Spotify AB.

Licensed under the [MIT License](LICENSE).
