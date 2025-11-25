# Phase 3: Advanced Features

**Goal:** Add batch downloads, playlists, history, and clipboard integration

**Timeline:** Week 4

**Milestone:** Power-user features complete

---

## 3.1 Batch Downloads

### Requirements

- [ ] Accept text file with one URL per line
- [ ] CLI flag: `--batch <FILE>` or `-b <FILE>`
- [ ] Read URLs from file
- [ ] Skip empty lines and comments (#)
- [ ] Validate all URLs before starting
- [ ] Support concurrent downloads (configurable)

### Concurrent Download Manager

- [ ] Download multiple videos simultaneously
- [ ] Configurable concurrency limit (default: 3)
- [ ] Queue management for pending downloads
- [ ] Individual progress tracking per download
- [ ] Overall batch progress tracking
- [ ] Error handling: skip failed, continue others
- [ ] Option to stop on first error

### Batch Progress UI

- [ ] Show list of all downloads in batch
- [ ] Show status for each (pending, downloading, complete, failed)
- [ ] Show progress bar for each active download
- [ ] Show overall batch progress (X/N complete)
- [ ] Show total data downloaded
- [ ] Scrollable list if many items
- [ ] Update in real-time

### CLI Features

- [ ] `ytdl --batch urls.txt`
- [ ] `ytdl --batch urls.txt --concurrent 5`
- [ ] `ytdl --batch urls.txt --stop-on-error`

---

## 3.2 Configuration File Support

### Configuration Location

- [ ] Default: `~/.config/ytdl/config.toml` (Linux/macOS)
- [ ] Windows: `%APPDATA%\ytdl\config.toml`
- [ ] Support custom path: `--config <FILE>`
- [ ] Create default config if not exists
- [ ] Load config on startup
- [ ] Merge config with CLI arguments (CLI takes precedence)

### Configuration Structure

- [ ] `[download]` section:
  - `output_dir` - Default output directory
  - `default_quality` - Default video quality
  - `concurrent_downloads` - Max concurrent downloads
  - `audio_format` - Audio format (mp3, m4a, flac)
  - `audio_quality` - Audio bitrate (192, 256, 320)
- [ ] `[ui]` section:
  - `theme` - UI theme (dark, light, auto)
  - `show_animations` - Enable/disable animations
  - `confirm_before_download` - Ask before starting
- [ ] `[advanced]` section:
  - `rate_limit` - Max download speed (e.g., "10M")
  - `retries` - Number of retry attempts
  - `timeout` - Download timeout in seconds
  - `use_cookies` - Use browser cookies

### Config Commands

- [ ] `ytdl config` - Show current configuration
- [ ] `ytdl config --edit` - Open config in editor
- [ ] `ytdl config --reset` - Reset to defaults
- [ ] `ytdl config --path` - Show config file path

---

## 3.3 Download History

### History Storage

- [ ] Store in JSON file: `~/.local/share/ytdl/history.json`
- [ ] Each entry contains:
  - URL
  - Video title
  - File path
  - File size
  - Timestamp (when downloaded)
  - Quality/format used
- [ ] Append to history after successful download
- [ ] Load history on startup for recent downloads display

### History Features

- [ ] Track all successful downloads
- [ ] Deduplicate: detect if URL already downloaded
- [ ] Option to skip already-downloaded videos
- [ ] Search history by title or URL
- [ ] Export history to CSV or JSON
- [ ] Clear old entries (by date or count)

### History Commands

- [ ] `ytdl history` - Show recent downloads (last 10)
- [ ] `ytdl history --limit 50` - Show last 50
- [ ] `ytdl history --search "keyword"` - Search history
- [ ] `ytdl history --export history.csv` - Export to CSV
- [ ] `ytdl clear-history` - Clear all history
- [ ] `ytdl clear-history --older-than 30d` - Clear old entries

### History UI Integration

- [ ] Show recent downloads in URL input screen
- [ ] Warning if URL already downloaded
- [ ] Option to re-download or skip
- [ ] View full history in TUI (new screen)

---

## 3.4 Playlist Support

### Playlist Detection

- [ ] Detect playlist URLs automatically
- [ ] Support formats:
  - `youtube.com/playlist?list=...`
  - `youtube.com/watch?v=...&list=...`
- [ ] Fetch playlist metadata
- [ ] Show total video count

### Playlist Features

- [ ] Download all videos in playlist
- [ ] Option to select specific videos
- [ ] Organize in subfolder by playlist name
- [ ] Sequential or concurrent download
- [ ] Resume interrupted playlists
- [ ] Skip already-downloaded videos

### Playlist UI

- [ ] New screen: Playlist Selection
- [ ] List all videos in playlist
- [ ] Checkbox to select/deselect each video
- [ ] "Select All" / "Deselect All" buttons
- [ ] Show video index, title, duration
- [ ] Show total download size estimate
- [ ] Proceed to batch download

### CLI Features

- [ ] `ytdl -p "playlist_url"` - Download all
- [ ] `ytdl -p "playlist_url" --select` - Interactive selection
- [ ] `ytdl -p "playlist_url" --range 1-10` - Download range
- [ ] `ytdl -p "playlist_url" --folder "My Playlist"` - Custom folder

---

## 3.5 Clipboard Integration

### Clipboard Detection

- [ ] Add `clipboard` dependency for cross-platform support
- [ ] Detect YouTube URL in clipboard
- [ ] Auto-fill URL input if clipboard has valid URL
- [ ] Visual indicator when clipboard has URL

### Clipboard Features

- [ ] Read clipboard on app start (optional)
- [ ] Paste shortcut in URL input (Ctrl+V)
- [ ] Watch mode: monitor clipboard for URLs
- [ ] Auto-download when URL detected (watch mode)
- [ ] Copy download link to clipboard (reverse)

### Watch Mode

- [ ] `ytdl --watch` - Monitor clipboard continuously
- [ ] Runs in background
- [ ] Auto-download when YouTube URL copied
- [ ] Notification on download start/complete
- [ ] Stop with Ctrl+C or specific signal

### CLI Features

- [ ] `ytdl --clipboard` - Use URL from clipboard
- [ ] `ytdl --watch` - Watch clipboard mode
- [ ] `ytdl --watch --auto-download` - Auto-download from clipboard

---

## 3.6 Resume Downloads

### Resume Support

- [ ] Detect partially downloaded files
- [ ] Use yt-dlp's resume capability
- [ ] Show option to resume or restart
- [ ] Handle corrupted partial files
- [ ] Clean up failed partial downloads

### Implementation

- [ ] Check for `.part` files before download
- [ ] Verify partial file integrity
- [ ] Resume using yt-dlp `--continue` flag
- [ ] UI indication of resumed download
- [ ] Log resume attempts

---

## 3.7 Advanced URL Support

### Multiple URL Sources

- [ ] Accept multiple URLs as arguments
- [ ] `ytdl "URL1" "URL2" "URL3"`
- [ ] Read from stdin: `cat urls.txt | ytdl -`
- [ ] Mix of individual URLs and playlists
- [ ] Handle duplicates automatically

### URL Formats

- [ ] Support shortened youtu.be URLs
- [ ] Support URLs with timestamps
- [ ] Support mobile URLs (m.youtube.com)
- [ ] Support embedded URLs
- [ ] Extract URL from text (find first valid URL)

---

## Testing

### Feature Testing Checklist

- [ ] Batch download with 5 URLs
- [ ] Concurrent downloads (3 simultaneous)
- [ ] Config file loading and saving
- [ ] History tracking and display
- [ ] History deduplication
- [ ] Playlist detection and download
- [ ] Playlist video selection
- [ ] Clipboard URL detection
- [ ] Clipboard paste in TUI
- [ ] Resume partial download
- [ ] Multiple URL arguments

### Integration Testing

- [ ] Config + batch download
- [ ] History + deduplication
- [ ] Playlist + concurrent downloads
- [ ] Clipboard + TUI integration
- [ ] All features logged properly

---

## Success Criteria

- [ ] Batch downloads working with concurrency control
- [ ] Config file loads, saves, and applies correctly
- [ ] History tracks all downloads accurately
- [ ] History deduplication prevents re-downloads
- [ ] Playlist support fully functional
- [ ] Playlist selection UI working
- [ ] Clipboard integration working
- [ ] Resume capability implemented
- [ ] All features tested and working
- [ ] Logging covers all new features

---

## Deliverables

- [ ] Batch download implementation
- [ ] Configuration system
- [ ] History tracking system
- [ ] Playlist support
- [ ] Clipboard integration
- [ ] Resume functionality
- [ ] Updated documentation

---

## Next Steps

Once Phase 3 is complete, proceed to [Phase 4: Polish & UX](phase-04-polish-ux.md)
