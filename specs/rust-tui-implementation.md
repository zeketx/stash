# YouTube Downloader - Rust TUI Implementation Spec

## Project Vision

Transform the current Python-based YouTube downloader into a beautiful, modern terminal UI application built with Rust. The goal is to combine the simplicity and speed of CLI tools with the visual polish and user experience of GUI applications.

### Design Philosophy
- **Simple**: Paste URL → Download → Done
- **Beautiful**: Rich visual feedback, progress indicators, animations
- **Fast**: Rust performance with concurrent downloads
- **Polished**: Professional-grade UX that delights users

## Inspiration
- `lazygit` - Beautiful git TUI with intuitive navigation
- `bottom` - Gorgeous system monitor with real-time visualization
- `spotify-tui` - Clean, focused interface
- `gh` (GitHub CLI) - Modern, polished CLI with excellent UX

---

## Technology Stack

### Core Language
**Rust** - For performance, safety, and excellent CLI ecosystem

### Key Dependencies

| Library | Purpose | Why |
|---------|---------|-----|
| `ratatui` | Terminal UI framework | Industry standard for Rust TUIs, used by `bottom`, `gitui` |
| `indicatif` | Progress bars & spinners | Beautiful progress visualization |
| `clap` (v4) | CLI argument parsing | Modern, derive-based API with auto-completion |
| `tokio` | Async runtime | Concurrent downloads, non-blocking I/O |
| `crossterm` | Terminal manipulation | Cross-platform terminal control |
| `serde` / `serde_json` | Serialization | Parse yt-dlp JSON output, config files |
| `toml` | Config file format | User configuration |
| `anyhow` | Error handling | Ergonomic error propagation |
| `colored` | Terminal colors | Rich CLI output |
| `directories` | Platform paths | Cross-platform config/download directories |

### External Dependencies
- **yt-dlp** - Called as subprocess for actual downloads
  - Proven, reliable, actively maintained
  - Handles YouTube API changes
  - Subprocess approach keeps our app simple

---

## Feature Requirements

### Phase 1: Foundation (MVP)

#### 1.1 Project Setup
- [x] Initialize Cargo project
- [x] Configure dependencies
- [x] Set up project structure:
  ```
  src/
  ├── main.rs           # Entry point, CLI parsing
  ├── downloader.rs     # yt-dlp wrapper
  ├── ui/
  │   ├── mod.rs
  │   ├── app.rs        # Application state
  │   ├── screens.rs    # Different UI screens
  │   └── widgets.rs    # Custom widgets
  ├── config.rs         # Configuration management
  └── utils.rs          # Helpers
  ```

#### 1.2 Basic CLI Interface
```bash
# Quick download
ytdl "https://youtube.com/watch?v=..."

# With options
ytdl -a "https://youtube.com/..."           # Audio only
ytdl -o ~/Downloads "https://youtube.com/..." # Custom output
ytdl -q 720p "https://youtube.com/..."      # Specific quality
```

**Arguments:**
- `url` - YouTube URL (required)
- `-o, --output <DIR>` - Output directory
- `-q, --quality <QUALITY>` - Video quality (default: best)
- `-a, --audio-only` - Download audio only (MP3)
- `-p, --playlist` - Download as playlist
- `-i, --info` - Show video info without downloading
- `--interactive` - Launch TUI mode

#### 1.3 yt-dlp Integration
- Execute yt-dlp as subprocess
- Parse JSON output for:
  - Video metadata (title, duration, uploader)
  - Available formats
  - Download progress
- Handle errors gracefully
- Format selection logic

**Example yt-dlp commands:**
```bash
# Get video info
yt-dlp -J "URL"

# Download with progress JSON
yt-dlp --newline --progress --progress-template '%(progress)j' "URL"

# List formats
yt-dlp -F "URL"
```

#### 1.4 Progress Visualization
Using `indicatif`:
- Download speed (MB/s)
- Progress percentage
- ETA (estimated time)
- File size (current/total)

```
Downloading: Amazing Video.mp4
[████████████████░░░░] 75% (512MB/682MB) 8.2MB/s ETA 00:21
```

#### 1.5 Error Handling
- URL validation (regex check before calling yt-dlp)
- Network error detection and retry logic
- Disk space checking
- FFmpeg dependency check (for audio conversion)
- Clear, actionable error messages

**Error Message Examples:**
```
Error: Invalid YouTube URL
→ Expected format: https://youtube.com/watch?v=... or https://youtu.be/...

Error: FFmpeg not found
→ Audio conversion requires FFmpeg
→ Install: brew install ffmpeg

Error: Insufficient disk space (need 1.2GB, have 500MB)
→ Free up space or change output directory with -o
```

---

### Phase 2: Beautiful TUI

#### 2.1 Interactive Mode
Launch with `ytdl --interactive` or just `ytdl` (no args)

**Screen Flow:**
```
[Welcome] → [URL Input] → [Format Selection] → [Downloading] → [Success]
     ↓                                               ↓
  [Settings]                                    [Error Recovery]
```

#### 2.2 Welcome Screen
```
╔════════════════════════════════════════════════╗
║                                                ║
║         🎥  YouTube Downloader  📥            ║
║                                                ║
║         Built with Rust + Ratatui              ║
║                                                ║
╚════════════════════════════════════════════════╝

  [Enter] Start Download
  [S]     Settings
  [H]     Help
  [Q]     Quit
```

#### 2.3 URL Input Screen
```
╔════════════════════════════════════════════════╗
║  Paste YouTube URL                             ║
╠════════════════════════════════════════════════╣
║                                                ║
║  > https://youtube.com/watch?v=_________       ║
║                                                ║
║  [Enter] Continue  [Ctrl+V] Paste  [Esc] Back ║
╚════════════════════════════════════════════════╝

Recent Downloads:
  • Amazing Video.mp4 (5 mins ago)
  • Cool Tutorial.mp4 (2 hours ago)
```

**Features:**
- Auto-detect clipboard (optional)
- URL validation with visual feedback
- Recent downloads history
- Keyboard shortcuts

#### 2.4 Format Selection Screen
```
╔════════════════════════════════════════════════╗
║  Video: Amazing Tutorial                       ║
║  Uploader: CoolChannel  •  Duration: 10:34     ║
╠════════════════════════════════════════════════╣
║  Select Quality:                               ║
║                                                ║
║  ▶ Best (1080p60, 2.4GB)                      ║
║    1080p (1080p, 1.8GB)                       ║
║    720p (720p, 890MB)                         ║
║    480p (480p, 420MB)                         ║
║    Audio Only (MP3, 8.2MB)                    ║
║                                                ║
║  [↑↓] Navigate  [Enter] Select  [Esc] Back    ║
╚════════════════════════════════════════════════╝
```

**Features:**
- Arrow key navigation
- Show estimated file sizes
- Highlight recommended quality
- Quick select: 'a' for audio, 'b' for best

#### 2.5 Download Progress Screen
```
╔════════════════════════════════════════════════╗
║  Downloading: Amazing Tutorial.mp4             ║
╠════════════════════════════════════════════════╣
║                                                ║
║  [████████████████████░░░░░░░░] 73%           ║
║                                                ║
║  Downloaded: 1.75 GB / 2.4 GB                 ║
║  Speed: 8.2 MB/s                               ║
║  ETA: 00:01:21                                 ║
║                                                ║
║  [Ctrl+C] Cancel                               ║
╚════════════════════════════════════════════════╝
```

**Features:**
- Real-time updates (every 100ms)
- Smooth progress bar animation
- Dynamic color coding (green when complete)
- Cancel support

#### 2.6 Success Screen
```
╔════════════════════════════════════════════════╗
║  ✓ Download Complete!                          ║
╠════════════════════════════════════════════════╣
║                                                ║
║  File: Amazing Tutorial.mp4                    ║
║  Size: 2.4 GB                                  ║
║  Time: 4m 32s                                  ║
║  Location: ~/Downloads/                        ║
║                                                ║
║  [O] Open File                                 ║
║  [F] Open Folder                               ║
║  [N] New Download                              ║
║  [Q] Quit                                      ║
╚════════════════════════════════════════════════╝
```

**Features:**
- Success animation (checkmark)
- Quick actions to open file/folder
- Download statistics
- Option to start new download

---

### Phase 3: Advanced Features

#### 3.1 Batch Downloads
```bash
# From file
ytdl --batch urls.txt

# Multiple URLs
ytdl "URL1" "URL2" "URL3"
```

**TUI Mode:**
```
╔════════════════════════════════════════════════╗
║  Batch Download (3 videos)                     ║
╠════════════════════════════════════════════════╣
║                                                ║
║  ✓ Video 1.mp4        [████████████] 100%     ║
║  ▶ Video 2.mp4        [██████░░░░░░]  52%     ║
║    Video 3.mp4        [░░░░░░░░░░░░]   0%     ║
║                                                ║
║  Overall: 2/3 complete  •  Total: 6.8 GB      ║
╚════════════════════════════════════════════════╝
```

**Features:**
- Concurrent downloads (configurable)
- Per-video progress bars
- Overall batch progress
- Skip on error option
- Pause/resume individual downloads

#### 3.2 Configuration File
Location: `~/.config/ytdl/config.toml`

```toml
[download]
output_dir = "~/Downloads/YouTube"
default_quality = "1080p"
concurrent_downloads = 3
audio_format = "mp3"
audio_quality = "192"

[ui]
theme = "dark"  # dark, light, auto
show_animations = true
confirm_before_download = false

[advanced]
rate_limit = "10M"  # Max download speed
retries = 3
timeout = 300
use_cookies = false
```

**CLI Override:**
```bash
ytdl --config ~/custom-config.toml "URL"
```

#### 3.3 Download History
Location: `~/.local/share/ytdl/history.json`

```json
{
  "downloads": [
    {
      "url": "https://youtube.com/watch?v=...",
      "title": "Amazing Video",
      "file_path": "~/Downloads/Amazing Video.mp4",
      "size": 2516582400,
      "timestamp": "2025-11-24T10:30:00Z",
      "quality": "1080p"
    }
  ]
}
```

**Features:**
- Track all downloads
- Deduplicate (skip already downloaded)
- View history in TUI
- Export history to CSV/JSON
- Clear old entries

#### 3.4 Clipboard Integration
```bash
# Auto-detect clipboard
ytdl --clipboard

# Watch clipboard (daemon mode)
ytdl --watch
```

**TUI Integration:**
- Auto-paste from clipboard in URL input
- Visual indicator when clipboard contains YouTube URL
- Quick paste shortcut (Ctrl+V)

#### 3.5 Playlist Support
```bash
ytdl -p "https://youtube.com/playlist?list=..."
```

**Features:**
- Show playlist info (total videos, duration)
- Select which videos to download (all, range, specific)
- Organize in subfolder by playlist name
- Resume interrupted playlists

**TUI Playlist Selection:**
```
╔════════════════════════════════════════════════╗
║  Playlist: Cool Tutorials (24 videos)          ║
╠════════════════════════════════════════════════╣
║                                                ║
║  [x] 1. Introduction (5:23)                    ║
║  [x] 2. Getting Started (8:45)                 ║
║  [ ] 3. Advanced Topics (15:30)                ║
║  [x] 4. Conclusion (3:12)                      ║
║                                                ║
║  [Space] Toggle  [A] All  [N] None  [Enter] OK║
╚════════════════════════════════════════════════╝
```

---

### Phase 4: Polish & UX

#### 4.1 Themes & Styling
- Dark mode (default)
- Light mode
- Auto (follows system)
- Custom color schemes

**Theme Definition:**
```rust
struct Theme {
    primary: Color,
    secondary: Color,
    success: Color,
    error: Color,
    background: Color,
    text: Color,
}
```

#### 4.2 Animations
- Smooth progress bar updates
- Loading spinners during metadata fetch
- Success checkmark animation
- Fade transitions between screens

#### 4.3 Keyboard Shortcuts
Global:
- `q` - Quit
- `h` - Help
- `?` - Show shortcuts
- `Esc` - Back/Cancel
- `Ctrl+C` - Interrupt

Context-specific:
- `Enter` - Confirm/Continue
- `↑/↓` - Navigate lists
- `Space` - Toggle selection
- `a` - Select all
- `n` - New download
- `o` - Open file
- `f` - Open folder

#### 4.4 Help System
```bash
ytdl --help     # Show usage
ytdl --examples # Show examples
```

**In-app help (press `h`):**
```
╔════════════════════════════════════════════════╗
║  Help & Shortcuts                              ║
╠════════════════════════════════════════════════╣
║                                                ║
║  Navigation:                                   ║
║    ↑/↓        Navigate lists                   ║
║    Enter      Select/Continue                  ║
║    Esc        Go back                          ║
║                                                ║
║  Actions:                                      ║
║    n          New download                     ║
║    s          Settings                         ║
║    h          Show this help                   ║
║    q          Quit                             ║
║                                                ║
║  [Esc] Close                                   ║
╚════════════════════════════════════════════════╝
```

#### 4.5 Notifications
- Desktop notifications on completion (macOS)
- Sound on success/error (optional)
- Terminal bell

---

### Phase 5: Distribution & Tooling

#### 5.1 Build & Release
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Cross-compilation
cargo build --target x86_64-apple-darwin
cargo build --target aarch64-apple-darwin
cargo build --target x86_64-unknown-linux-gnu
cargo build --target x86_64-pc-windows-gnu
```

**Binary Size Optimization:**
```toml
[profile.release]
opt-level = "z"      # Optimize for size
lto = true           # Link-time optimization
codegen-units = 1    # Single codegen unit
strip = true         # Strip symbols
panic = "abort"      # Smaller panic handler
```

#### 5.2 Installation Methods

**Cargo:**
```bash
cargo install ytdl
```

**Homebrew (macOS/Linux):**
```bash
brew tap yourusername/ytdl
brew install ytdl
```

**Direct Download:**
```bash
# macOS (Apple Silicon)
curl -L https://github.com/user/ytdl/releases/latest/download/ytdl-aarch64-apple-darwin -o ytdl
chmod +x ytdl
sudo mv ytdl /usr/local/bin/

# macOS (Intel)
curl -L https://github.com/user/ytdl/releases/latest/download/ytdl-x86_64-apple-darwin -o ytdl
chmod +x ytdl
sudo mv ytdl /usr/local/bin/

# Linux
curl -L https://github.com/user/ytdl/releases/latest/download/ytdl-x86_64-unknown-linux-gnu -o ytdl
chmod +x ytdl
sudo mv ytdl /usr/local/bin/
```

#### 5.3 Shell Completions
Generate completions with `clap_complete`:

```bash
# Generate for your shell
ytdl --completions bash > /usr/local/etc/bash_completion.d/ytdl
ytdl --completions zsh > /usr/local/share/zsh/site-functions/_ytdl
ytdl --completions fish > ~/.config/fish/completions/ytdl.fish
```

#### 5.4 CI/CD (GitHub Actions)
```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      - run: cargo build --release
      - uses: actions/upload-artifact@v3
```

**Release Checklist:**
- [ ] Run tests
- [ ] Update CHANGELOG.md
- [ ] Bump version in Cargo.toml
- [ ] Create git tag
- [ ] GitHub release with binaries
- [ ] Update Homebrew formula

---

## Architecture Overview

### Application Flow
```
main.rs
  ├─> CLI args parsed (clap)
  ├─> Interactive mode?
  │     ├─> Yes: Launch TUI (ratatui)
  │     │         └─> Event loop
  │     │               ├─> Handle input
  │     │               ├─> Update state
  │     │               └─> Render UI
  │     └─> No: Direct download
  │             └─> Call downloader
  │
downloader.rs
  └─> Spawn yt-dlp subprocess
        ├─> Parse JSON output
        ├─> Update progress
        └─> Return result
```

### Module Breakdown

**main.rs** (~200 lines)
- Entry point
- CLI parsing with `clap`
- Mode selection (CLI vs TUI)

**downloader.rs** (~300 lines)
- `YtDlp` struct - subprocess wrapper
- Progress parsing from JSON
- Error handling
- Format selection

**config.rs** (~150 lines)
- Config file loading/saving
- Default values
- Config validation

**ui/app.rs** (~200 lines)
- Application state
- State transitions
- Event handling

**ui/screens.rs** (~400 lines)
- Individual screen implementations
- Input handling per screen
- Screen-specific rendering

**ui/widgets.rs** (~200 lines)
- Custom widgets (progress bars, etc.)
- Reusable UI components

**utils.rs** (~100 lines)
- File size formatting
- Duration formatting
- URL validation
- Clipboard integration

### State Management
```rust
enum AppState {
    Welcome,
    UrlInput { url: String },
    FetchingInfo { url: String },
    FormatSelection { video_info: VideoInfo },
    Downloading { progress: DownloadProgress },
    Success { download_result: DownloadResult },
    Error { error_message: String },
}

struct App {
    state: AppState,
    config: Config,
    history: History,
}
```

---

## Implementation Timeline

### Week 1: Foundation
- [x] Project setup
- [ ] Basic CLI parsing
- [ ] yt-dlp wrapper (basic)
- [ ] Simple progress output

**Milestone:** Working CLI downloader

### Week 2: Core Features
- [ ] Format selection
- [ ] Error handling
- [ ] Config file support
- [ ] Progress bars with indicatif

**Milestone:** Feature parity with Python version

### Week 3: TUI Development
- [ ] Ratatui setup
- [ ] Welcome & URL input screens
- [ ] Format selection screen
- [ ] Download progress screen

**Milestone:** Interactive TUI mode working

### Week 4: Polish
- [ ] Success/error screens
- [ ] Keyboard shortcuts
- [ ] Help system
- [ ] Themes

**Milestone:** Beautiful, polished TUI

### Week 5: Advanced Features
- [ ] Batch downloads
- [ ] Playlist support
- [ ] History tracking
- [ ] Clipboard integration

**Milestone:** Advanced feature set

### Week 6: Release
- [ ] Testing & bug fixes
- [ ] Documentation
- [ ] Cross-platform builds
- [ ] GitHub release
- [ ] Homebrew formula

**Milestone:** v1.0 release

---

## Success Criteria

### Must Have (v1.0)
- ✅ Simple paste-and-go workflow
- ✅ Beautiful progress visualization
- ✅ Interactive TUI mode
- ✅ Format selection
- ✅ Error recovery
- ✅ Configuration file
- ✅ Cross-platform (macOS, Linux, Windows)

### Should Have (v1.1)
- 🎯 Batch downloads
- 🎯 Playlist support
- 🎯 Download history
- 🎯 Clipboard integration

### Nice to Have (v2.0)
- 💡 Desktop notifications
- 💡 Resume downloads
- 💡 Concurrent downloads
- 💡 Custom themes
- 💡 Subtitle downloads

---

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_url_validation() { }

    #[test]
    fn test_format_parsing() { }

    #[test]
    fn test_progress_calculation() { }
}
```

### Integration Tests
```rust
#[test]
fn test_download_workflow() {
    // Mock yt-dlp subprocess
    // Verify full download flow
}
```

### Manual Testing Checklist
- [ ] Download single video
- [ ] Download playlist
- [ ] Audio-only download
- [ ] Format selection
- [ ] Error scenarios (invalid URL, network error, etc.)
- [ ] Config file loading
- [ ] All TUI screens
- [ ] Keyboard shortcuts

---

## Future Enhancements

### v2.0 Ideas
- Search YouTube from CLI
- Download thumbnails
- Subtitle support
- Custom post-processing scripts
- Queue management (add to queue, process later)
- Integration with media libraries (Plex, Jellyfin)

### Community Features
- Plugin system for custom extractors
- Configurable hotkeys
- Export download statistics
- API mode (REST server)

---

## References

### Documentation
- [Ratatui Book](https://ratatui.rs/)
- [yt-dlp Documentation](https://github.com/yt-dlp/yt-dlp)
- [Clap Derive Tutorial](https://docs.rs/clap/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

### Example Projects
- [gitui](https://github.com/extrawurst/gitui) - Git TUI
- [bottom](https://github.com/ClementTsang/bottom) - System monitor TUI
- [spotify-tui](https://github.com/Rigellute/spotify-tui) - Spotify TUI
- [bat](https://github.com/sharkdp/bat) - Modern cat alternative
- [ripgrep](https://github.com/BurntSushi/ripgrep) - Fast grep alternative

### Design Inspiration
- [Charm.sh](https://charm.sh/) - Beautiful CLI tools
- [Bubbletea](https://github.com/charmbracelet/bubbletea) - Go TUI framework

---

## Notes

- **Keep it simple**: Don't over-engineer. Ship v1.0 with core features, iterate.
- **User feedback**: Get early feedback on TUI design before implementing all features.
- **Performance**: Profile download performance, ensure no bottlenecks.
- **Cross-platform**: Test on all platforms early and often.
- **Documentation**: Write README and docs as you build, not after.

---

**Document Version:** 1.0
**Created:** 2025-11-24
**Status:** Planning Phase
**Next Review:** After Phase 1 completion
