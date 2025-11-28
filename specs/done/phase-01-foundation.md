# Phase 1: Foundation (MVP)

**Goal:** Build a working CLI downloader with comprehensive logging and observability

**Milestone:** Feature parity with Python version + observable logs at all levels

---

## 1.1 Project Setup

### Requirements

- ✅ Initialize Cargo project with proper structure
- ✅ Configure all required dependencies in Cargo.toml
- ✅ Set up project directory structure:
  - Main entry point
  - Downloader module
  - Logger module
  - Error types module
  - Config module
  - Utils module
- ✅ Create .gitignore for Rust project
- [ ] Initialize README.md

### Logging Requirements (CRITICAL)

- ✅ Implement comprehensive logging system using `tracing`
- ✅ Support multiple log levels: TRACE, DEBUG, INFO, WARN, ERROR
- ✅ Log to both console (stdout) and file
- ✅ File logging with daily rotation
- ✅ Configurable via CLI flags: `-v`, `-vv`, `-vvv`
- ✅ Configurable via environment variables: `YTDL_LOG_LEVEL`, `YTDL_LOG_FILE`, `YTDL_LOG_JSON`
- ✅ JSON log format option for machine parsing
- ✅ Human-friendly pretty format for console
- ✅ Include timestamps, line numbers, module paths, thread IDs
- ✅ Log application startup information (version, OS, args, working directory)
- ✅ Log all state transitions and major operations
- ✅ Default log directory: `~/.local/share/ytdl/logs/`

### Dependencies Required

- ✅ CLI: `clap` (with derive features)
- ✅ Async: `tokio` (full features)
- ✅ Errors: `anyhow`, `thiserror`
- ✅ Logging: `tracing`, `tracing-subscriber`, `tracing-appender`, `tracing-log`
- ✅ Progress: `indicatif` (with tokio features)
- ✅ Serialization: `serde`, `serde_json`, `toml`
- ✅ Paths: `directories`
- ✅ Terminal: `colored`, `crossterm`

---

## 1.2 Basic CLI Interface

### Command Structure

- ✅ Primary command accepts YouTube URL as positional argument
- ✅ Support for running without arguments (launches interactive mode)

### Required Arguments & Flags

- ✅ `url` - YouTube video or playlist URL (optional, prompts if missing)
- ✅ `-o, --output <DIR>` - Output directory (default: `./downloads`)
- ✅ `-q, --quality <QUALITY>` - Video quality (default: `best`)
- ✅ `-a, --audio-only` - Download audio only as MP3
- ✅ `-p, --playlist` - Download as playlist
- ✅ `-i, --info` - Show video information without downloading
- ✅ `--interactive` - Launch TUI mode
- ✅ `-b, --batch <FILE>` - Batch download from file
- ✅ `-v, --verbose` - Verbose logging (repeatable: -v, -vv, -vvv)
- ✅ `-q, --quiet` - Quiet mode (errors only)
- ✅ `--log-file` - Enable file logging
- ✅ `--log-json` - Use JSON log format
- ✅ `--config <FILE>` - Custom config file path

### Subcommands

- ✅ `config` - Show current configuration
- ✅ `history` - Show download history
- ✅ `clear-history` - Clear download history
- ✅ `completions <SHELL>` - Generate shell completions

### Environment Variables

- ✅ `YTDL_OUTPUT_DIR` - Default output directory
- ✅ `YTDL_LOG_LEVEL` - Log level (trace/debug/info/warn/error)
- ✅ `YTDL_LOG_FILE` - Enable/disable file logging
- ✅ `YTDL_LOG_JSON` - Enable JSON logging

---

## 1.3 yt-dlp Integration

### Requirements

- ✅ Detect yt-dlp binary in PATH
- ✅ Display helpful error if yt-dlp not found (with installation instructions)
- ✅ Execute yt-dlp as subprocess
- ✅ Check yt-dlp version on startup
- ✅ Log yt-dlp version and path

### Video Information Fetching

- ✅ Fetch video metadata using `yt-dlp --dump-json`
- ✅ Parse JSON output to extract:
  - Video ID
  - Title
  - Uploader
  - Duration
  - View count
  - Upload date
  - Available formats
  - Thumbnail URL
  - Description
- ✅ Handle parsing errors gracefully
- ✅ Log all metadata operations

### Format Handling

- ✅ List all available formats
- ✅ Parse format information (resolution, codec, filesize, fps)
- ✅ Support quality selection by resolution (1080p, 720p, 480p, etc.)
- ✅ Support "best" quality automatic selection
- ✅ Handle format unavailability errors

### Download Execution

- ✅ Execute yt-dlp with appropriate arguments
- ✅ Stream progress output in real-time
- ✅ Parse progress information from yt-dlp output
- ✅ Support audio-only downloads with format conversion
- ✅ Handle download interruption (Ctrl+C)
- ✅ Verify download completion
- ✅ Log all download operations with full details

---

## 1.4 Progress Visualization

### Requirements

- ✅ Display download progress using `indicatif` progress bars
- ✅ Show percentage complete
- ✅ Show download speed (MB/s)
- ✅ Show estimated time remaining (ETA)
- ✅ Show downloaded bytes / total bytes
- ✅ Use colored progress bar (green when complete)
- ✅ Smooth progress updates (no flickering)
- ✅ Support quiet mode (suppress progress)
- ✅ Log progress updates at DEBUG level

### Progress Bar Features

- ✅ Animated spinner during metadata fetch
- ✅ Full progress bar during download
- ✅ Success checkmark on completion
- ✅ Support for terminal resize
- ✅ Clear progress bar on error

---

## 1.5 Error Handling

### Error Types Required

- ✅ Invalid YouTube URL
- ✅ yt-dlp not found
- ✅ yt-dlp execution failed
- ✅ FFmpeg not found (for audio conversion)
- ✅ Insufficient disk space
- ✅ Network errors (timeout, connection failed)
- ✅ Parse errors (JSON, progress output)
- ✅ IO errors (file write, permissions)
- ✅ Configuration errors

### Error Display Requirements

- ✅ User-friendly error messages in red
- ✅ Actionable suggestions for each error type
- ✅ Installation instructions for missing dependencies
- ✅ Example of correct URL format for invalid URLs
- ✅ Log full error details at ERROR level
- ✅ Log stack traces at DEBUG level

### URL Validation

- ✅ Validate URL format before calling yt-dlp
- ✅ Support youtube.com/watch?v=... format
- ✅ Support youtu.be/... format
- ✅ Support youtube.com/playlist?list=... format
- ✅ Display specific error for invalid URLs

### Dependency Checks

- ✅ Check for yt-dlp on startup
- ✅ Check for FFmpeg when audio conversion requested
- ✅ Display installation instructions for missing tools
- ✅ Platform-specific instructions (macOS, Linux, Windows)

---

## 1.6 Observability & Logging

### Startup Logging

- ✅ Log application version
- ✅ Log command-line arguments
- ✅ Log operating system and architecture
- ✅ Log current working directory
- ✅ Log configuration source (file, defaults, env vars)
- ✅ Log yt-dlp version and path

### Operation Logging

- ✅ Log every user action (download start, format selection, etc.)
- ✅ Log all subprocess executions with full commands
- ✅ Log all file operations (create, write, delete)
- ✅ Log all network operations
- ✅ Log state transitions
- ✅ Log performance metrics (download duration, file size)

### Debug Logging

- ✅ Log all yt-dlp output at TRACE level
- ✅ Log all parsed data structures at DEBUG level
- ✅ Log all progress updates at DEBUG level
- ✅ Log all configuration values at startup

### Log File Management

- ✅ Daily log file rotation
- ✅ Log files named: `ytdl-YYYY-MM-DD.log`
- ✅ Keep logs in: `~/.local/share/ytdl/logs/` (Linux/macOS)
- ✅ No automatic log cleanup (user responsibility)
- ✅ Both console and file logging active simultaneously

---

## Testing

### Unit Tests Required

- ✅ URL validation tests
- ✅ Format parsing tests
- ✅ Progress calculation tests
- ✅ Error message formatting tests
- ✅ Configuration loading tests

### Integration Tests Required

- ✅ yt-dlp detection test
- ✅ Version check test
- ✅ Mock download test
- ✅ Error handling test

### Manual Testing Checklist

- ✅ Download single video (default quality)
- ✅ Download with specific quality
- ✅ Download audio only
- ✅ Show video info
- ✅ Invalid URL handling
- ✅ Missing yt-dlp handling
- ✅ Test all verbosity levels (-v, -vv, -vvv)
- ✅ Test quiet mode
- ✅ Test file logging
- ✅ Test JSON logging
- ✅ Verify logs are observable and useful

---

## Success Criteria

- ✅ Project compiles without errors
- ✅ All dependencies installed and working
- ✅ Basic download functionality working
- ✅ Progress bars display correctly
- ✅ Error handling working with helpful messages
- ✅ Comprehensive logging implemented
- ✅ Logs observable at INFO level by default
- ✅ Logs observable at DEBUG level with -v
- ✅ Logs observable at TRACE level with -vv
- ✅ Log files created and rotated properly
- ✅ All manual tests passing
- ✅ Feature parity with Python version achieved

---

## Deliverables

- ✅ Working Rust CLI binary
- ✅ Passing all tests
- ✅ Comprehensive logging throughout
- [ ] Documentation for logging system
- [ ] README with usage examples

---
