# ytdl - YouTube Downloader

A feature-rich YouTube downloader with both CLI and TUI interfaces, built in Rust. Download videos, playlists, and manage your download history with ease.

## Features

- 🎨 **Beautiful TUI** - Interactive terminal interface with animations
- 📦 **Batch Downloads** - Download multiple videos concurrently
- 📝 **History Tracking** - Keep track of all your downloads
- 📋 **Clipboard Support** - Auto-detect YouTube URLs from clipboard
- 🎵 **Audio Extraction** - Download as MP3 with automatic conversion
- 🎬 **Playlist Support** - Download entire playlists or specific ranges
- 🔔 **Notifications** - Get notified when downloads complete
- ⚙️ **Configurable** - Customize output directory, quality, and more

## Quick Start

### Installation

1. **Install dependencies:**
   ```bash
   # yt-dlp (required)
   pip install yt-dlp

   # FFmpeg (optional, for audio conversion)
   brew install ffmpeg  # macOS
   ```

2. **Build from source:**
   ```bash
   cargo build --release
   ```

3. **Run:**
   ```bash
   ./target/release/ytdl --help
   ```

## Usage

### Interactive Mode (TUI)

The easiest way to get started:

```bash
ytdl -i
```

This launches the interactive interface where you can:
- Enter YouTube URLs with visual validation
- Preview video information before downloading
- Select quality and format options
- Watch download progress in real-time
- Get helpful suggestions if something goes wrong

### Command Line Interface

#### Basic Downloads

```bash
# Download a video
ytdl "https://youtube.com/watch?v=dQw4w9WgXcQ"

# Download audio only
ytdl -a "https://youtube.com/watch?v=dQw4w9WgXcQ"

# Download to specific directory
ytdl -o ~/Videos "https://youtube.com/watch?v=dQw4w9WgXcQ"

# Download in specific quality
ytdl -q 720 "https://youtube.com/watch?v=dQw4w9WgXcQ"
```

#### Playlists

```bash
# Download entire playlist
ytdl -p "https://youtube.com/playlist?list=PLAYLIST_ID"

# Download specific range (videos 1-5)
ytdl --range 1-5 "https://youtube.com/playlist?list=PLAYLIST_ID"

# Download playlist to custom folder
ytdl -p --folder "My Playlist" "https://youtube.com/playlist?list=PLAYLIST_ID"
```

#### Batch Downloads

```bash
# Create a file with URLs (one per line)
cat > urls.txt << EOF
https://youtube.com/watch?v=VIDEO1
https://youtube.com/watch?v=VIDEO2
https://youtube.com/watch?v=VIDEO3
EOF

# Download all URLs
ytdl -b urls.txt

# Download with 5 concurrent downloads
ytdl -b urls.txt --concurrent 5

# Stop on first error
ytdl -b urls.txt --stop-on-error
```

#### Clipboard Integration

```bash
# Download from clipboard
ytdl --clipboard

# Watch clipboard and auto-download
ytdl --watch
```

#### History Management

```bash
# View download history
ytdl history

# Search history
ytdl history --search "tutorial"

# Limit results
ytdl history --limit 20

# Export to CSV
ytdl history --export history.csv

# Clear history
ytdl clear-history

# Clear entries older than 30 days
ytdl clear-history --older-than 30
```

#### Configuration

```bash
# View current configuration
ytdl config

# Use custom config file
ytdl --config ~/.ytdl/config.toml "URL"
```

### Examples

For more examples:

```bash
ytdl --examples
```

## Project Structure

The codebase follows a clean, layered architecture:

```
src/
├── core/          # Business logic
│   ├── batch.rs      - Concurrent batch downloads
│   ├── history.rs    - Download history tracking
│   └── playlist.rs   - Playlist management
│
├── infra/         # External integrations
│   ├── downloader.rs    - yt-dlp wrapper
│   ├── clipboard.rs     - Clipboard monitoring
│   ├── logger.rs        - Logging setup
│   └── notifications.rs - Desktop notifications
│
├── cli/           # Command-line interface
│   ├── commands/     - Individual command handlers
│   ├── config.rs     - Configuration management
│   └── parser.rs     - Argument parsing
│
├── tui/           # Terminal UI
│   ├── screens/      - UI screens (welcome, download, etc.)
│   ├── widgets/      - Reusable UI components
│   ├── app.rs        - Application state
│   └── runner.rs     - Main event loop
│
└── shared/        # Shared utilities
    ├── constants.rs  - Application constants
    ├── error.rs      - Error types
    └── utils.rs      - Helper functions
```

This structure makes the code:
- **Easy to navigate** - Find what you need quickly
- **Simple to test** - Each layer can be tested independently
- **Maintainable** - Changes are localized to specific modules
- **Extensible** - Add new features without breaking existing code

## Configuration

Create a config file at `~/.ytdl/config.toml`:

```toml
# Output directory for downloads
output_dir = "~/Downloads/YouTube"

# Default video quality (best, 1080, 720, 480, etc.)
quality = "best"

# Download audio only by default
audio_only = false

# Number of concurrent downloads for batch mode
concurrent_downloads = 3

# Skip videos already in download history
skip_duplicates = true

# Logging level (error, warn, info, debug, trace)
log_level = "info"

# Enable file logging
enable_file_logging = false

# Use JSON format for logs
enable_json_logging = false
```

You can also use environment variables:

```bash
export YTDL_OUTPUT_DIR="~/Videos"
export YTDL_QUALITY="1080"
export YTDL_LOG_LEVEL="debug"
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_config_command
```

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run without installing
cargo run -- --help
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Fix common issues
cargo fix
```

## Notes

- Downloads go to `./downloads` by default (customizable)
- Playlists are organized into subfolders automatically
- Partial downloads can be resumed with `--resume`
- History is stored at `~/.ytdl/history.json`
- Logs are saved to `~/.ytdl/logs/` when enabled
- Always respect YouTube's Terms of Service

## License

MIT

## Contributing

Contributions welcome! Feel free to open issues or submit pull requests.
