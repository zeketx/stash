# Phase 2: Beautiful TUI

**Goal:** Build an interactive terminal UI with ratatui framework

**Timeline:** Weeks 2-3

**Milestone:** Polished TUI with all core screens functional and navigable

---

## 2.1 Ratatui Setup

### Requirements

- [ ] Add ratatui dependency to Cargo.toml
- [ ] Add crossterm for terminal manipulation
- [ ] Add tokio sync primitives for event handling
- [ ] Create UI module structure:
  - UI root module
  - Application state module
  - Event handling module
  - Screens submodule
  - Widgets submodule
  - Theme module

### Terminal Management

- [ ] Initialize terminal in raw mode
- [ ] Handle terminal cleanup on exit
- [ ] Support terminal resize events
- [ ] Handle Ctrl+C gracefully
- [ ] Restore terminal state on panic

---

## 2.2 Welcome Screen

### Layout Requirements

- [ ] Center-aligned welcome banner
- [ ] Application title with visual decoration
- [ ] Version information
- [ ] Menu with keyboard shortcuts
- [ ] Footer with "press any key" message

### Menu Options

- [ ] [Enter] Start Download
- [ ] [S] Settings
- [ ] [H] Help
- [ ] [Q] Quit

### Visual Requirements

- [ ] ASCII art logo or banner
- [ ] Colored text (cyan for titles, green for shortcuts)
- [ ] Border decoration around main content
- [ ] Responsive to terminal size

---

## 2.3 URL Input Screen

### Layout Requirements

- [ ] Title: "Paste YouTube URL"
- [ ] Large input box with visual focus
- [ ] Placeholder text showing example URL
- [ ] Live URL validation with visual feedback (green/red)
- [ ] Help text with keyboard shortcuts
- [ ] Recent downloads list (below input)
- [ ] Status message (valid/invalid URL)

### Input Features

- [ ] Text input with cursor
- [ ] Blinking cursor animation
- [ ] Support for paste (Ctrl+V)
- [ ] Support for clear (Ctrl+U)
- [ ] Character-by-character input
- [ ] Backspace support
- [ ] URL validation on each keystroke
- [ ] Visual indication of validation state

### Recent Downloads

- [ ] Show last 5-10 downloads
- [ ] Display video titles
- [ ] Show download time (relative: "5 mins ago")
- [ ] Optional: Make clickable to re-download

### Keyboard Shortcuts

- [ ] [Enter] - Continue (if valid URL)
- [ ] [Ctrl+V] - Paste from clipboard
- [ ] [Ctrl+U] - Clear input
- [ ] [Esc] - Go back to welcome
- [ ] [Backspace] - Delete character

---

## 2.4 Format Selection Screen

### Layout Requirements

- [ ] Video information box at top
- [ ] Format list in center (scrollable)
- [ ] Help text at bottom

### Video Information Display

- [ ] Video title (truncated if too long)
- [ ] Uploader name
- [ ] Duration
- [ ] View count (optional)
- [ ] Upload date (optional)

### Format List

- [ ] Show 5-10 format options
- [ ] Currently selected format highlighted
- [ ] Arrow indicator for selection
- [ ] Each format shows:
  - Quality label (Best, 1080p, 720p, 480p, Audio Only)
  - Resolution
  - Estimated file size
- [ ] Scroll support for many formats

### Selection Features

- [ ] Arrow key navigation (up/down)
- [ ] Visual highlight of selected item
- [ ] Quick select shortcuts:
  - [A] Audio only
  - [B] Best quality
  - [1] 1080p
  - [7] 720p
  - [4] 480p

### Keyboard Shortcuts

- [ ] [↑/↓] - Navigate options
- [ ] [Enter] - Select and continue
- [ ] [Esc] - Go back
- [ ] Quick select keys (a, b, 1, 7, 4)

---

## 2.5 Download Progress Screen

### Layout Requirements

- [ ] Title with video filename
- [ ] Large progress bar (center)
- [ ] Statistics panel (below progress bar)
- [ ] Cancel instruction at bottom

### Progress Bar

- [ ] Full-width progress indicator
- [ ] Percentage display (center of bar)
- [ ] Color coding:
  - Cyan/blue for in-progress
  - Green for complete
  - Red for error
- [ ] Smooth animation (no flickering)
- [ ] Update rate: 10-20 times per second

### Statistics Display

- [ ] Downloaded bytes / Total bytes
- [ ] Download speed (MB/s)
- [ ] Estimated time remaining (ETA)
- [ ] Elapsed time
- [ ] All values update in real-time

### Visual Features

- [ ] Animated progress bar fill
- [ ] Spinner animation during initialization
- [ ] Bold/colored numbers for stats
- [ ] Proper number formatting (2.45 GB, 8.2 MB/s)

### Keyboard Shortcuts

- [ ] [Ctrl+C] - Cancel download

---

## 2.6 Success Screen

### Layout Requirements

- [ ] Success message with checkmark (✓)
- [ ] File information panel
- [ ] Quick actions menu
- [ ] Footer with keyboard shortcuts

### File Information

- [ ] Filename
- [ ] File size
- [ ] Download duration
- [ ] Save location (full path)

### Quick Actions

- [ ] [O] Open file (launch default player)
- [ ] [F] Open folder (in file manager)
- [ ] [N] New download
- [ ] [Q] Quit application

### Visual Features

- [ ] Green color scheme for success
- [ ] Checkmark or success icon
- [ ] Clear action buttons with keyboard shortcuts
- [ ] Celebratory feel (without being obnoxious)

---

## 2.7 Error Screen

### Layout Requirements

- [ ] Error message with cross mark (✗)
- [ ] Error details panel
- [ ] Troubleshooting suggestions
- [ ] Recovery actions menu

### Error Information

- [ ] Error type/category
- [ ] Detailed error message
- [ ] Troubleshooting steps (bulleted list)
- [ ] Suggested actions

### Recovery Actions

- [ ] [R] Retry download
- [ ] [N] New download
- [ ] [Q] Quit application
- [ ] [H] Help/More info

### Visual Features

- [ ] Red color scheme for errors
- [ ] Cross mark or error icon
- [ ] Clear, actionable suggestions
- [ ] Links to documentation if applicable

---

## 2.8 Application State Machine

### State Definitions

- [ ] Welcome - Initial screen
- [ ] UrlInput - Entering YouTube URL
- [ ] FetchingInfo - Loading video metadata (with spinner)
- [ ] FormatSelection - Choosing quality/format
- [ ] Downloading - Download in progress
- [ ] Success - Download completed
- [ ] Error - Error occurred

### State Transitions

- [ ] Welcome → UrlInput (Enter pressed)
- [ ] Welcome → Settings (S pressed)
- [ ] Welcome → Help (H pressed)
- [ ] Welcome → Exit (Q pressed)
- [ ] UrlInput → FetchingInfo (valid URL, Enter pressed)
- [ ] UrlInput → Welcome (Esc pressed)
- [ ] FetchingInfo → FormatSelection (metadata loaded)
- [ ] FetchingInfo → Error (fetch failed)
- [ ] FormatSelection → Downloading (format selected)
- [ ] FormatSelection → UrlInput (Esc pressed)
- [ ] Downloading → Success (download completed)
- [ ] Downloading → Error (download failed)
- [ ] Downloading → UrlInput (cancelled)
- [ ] Success → UrlInput (N pressed)
- [ ] Success → Exit (Q pressed)
- [ ] Error → UrlInput (N pressed)
- [ ] Error → Downloading (R pressed - retry)
- [ ] Error → Exit (Q pressed)

### State Management

- [ ] Single AppState enum with all states
- [ ] State contains relevant data (URL, video info, progress, error)
- [ ] Clean transition methods
- [ ] State changes logged for debugging

---

## 2.9 Event Handling

### Event Loop

- [ ] Async event loop using tokio
- [ ] Poll for keyboard events
- [ ] Poll for progress updates
- [ ] Handle terminal resize
- [ ] Handle application state changes
- [ ] Render on state change or timeout

### Keyboard Event Handling

- [ ] Global shortcuts (work everywhere):
  - Q - Quit
  - H/? - Help
  - Esc - Back/Cancel
- [ ] Context-specific key handlers per screen
- [ ] Key event dispatching based on current state

### Update Rate

- [ ] Render at 60 FPS for smooth animations
- [ ] Throttle progress updates to avoid flickering
- [ ] Only redraw when state changes or timer expires

---

## 2.10 Themes

### Theme Support

- [ ] Define Theme struct with color palette
- [ ] Dark theme (default)
- [ ] Light theme
- [ ] Load theme from config
- [ ] Apply theme colors throughout UI

### Color Palette

- [ ] Primary color (for titles, highlights)
- [ ] Secondary color (for borders, less important)
- [ ] Success color (green)
- [ ] Error color (red)
- [ ] Warning color (yellow)
- [ ] Info color (blue/cyan)
- [ ] Background color
- [ ] Foreground/text color

---

## Testing

### Manual TUI Testing Checklist

- [ ] Welcome screen displays correctly
- [ ] Can navigate to URL input
- [ ] URL input accepts text
- [ ] URL validation works (red/green)
- [ ] Can navigate to format selection
- [ ] Format list displays correctly
- [ ] Arrow keys navigate formats
- [ ] Progress bar animates smoothly
- [ ] Progress stats update correctly
- [ ] Success screen shows correct info
- [ ] Error screen displays properly
- [ ] All keyboard shortcuts work
- [ ] Can quit from any screen with Q
- [ ] Esc goes back in navigation
- [ ] Terminal resizes handled gracefully
- [ ] Ctrl+C cancels cleanly

### Visual Testing

- [ ] All screens fit in 80x24 terminal
- [ ] All screens readable in larger terminals
- [ ] Colors display correctly
- [ ] No flickering or visual glitches
- [ ] Progress bar smooth
- [ ] Text properly aligned
- [ ] Borders render correctly

---

## Success Criteria

- [ ] All 6 screens implemented and rendering
- [ ] Smooth navigation between screens
- [ ] Keyboard shortcuts working correctly
- [ ] Real-time progress updates
- [ ] Responsive UI (no lag or freezing)
- [ ] Error screens helpful and actionable
- [ ] All state transitions working
- [ ] Event loop running smoothly
- [ ] Terminal cleanup on exit
- [ ] Logging integrated throughout TUI operations

---

## Deliverables

- [ ] Functional TUI with all screens
- [ ] State machine implementation
- [ ] Event handling system
- [ ] Theme system
- [ ] Comprehensive testing completed

---

## Next Steps

Once Phase 2 is complete, proceed to [Phase 3: Advanced Features](phase-03-advanced-features.md)
