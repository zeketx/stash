# Phase 4: Polish & UX

**Goal:** Add themes, animations, keyboard shortcuts, and help system

**Timeline:** Week 5

**Milestone:** Professional, polished user experience

---

## 4.1 Themes

- [ ] Dark theme (default)
- [ ] Light theme
- [ ] Auto theme (follows system)
- [ ] Theme struct with color palette
- [ ] Load theme from config file
- [ ] Apply consistently across all screens

---

## 4.2 Animations

- [ ] Smooth progress bar updates (no flickering)
- [ ] Loading spinners (⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏)
- [ ] Blinking cursor in input fields
- [ ] Success checkmark animation
- [ ] Fade transitions between screens (optional)
- [ ] 60 FPS render rate

---

## 4.3 Keyboard Shortcuts

### Global Shortcuts (work everywhere)
- [ ] `q` - Quit
- [ ] `h` or `?` - Help
- [ ] `Esc` - Back/Cancel
- [ ] `Ctrl+C` - Interrupt

### Context-Specific
- [ ] Document all shortcuts per screen
- [ ] Consistent behavior across app
- [ ] Display shortcuts in help overlay

---

## 4.4 Help System

- [ ] In-app help screen (press `h`)
- [ ] Show all keyboard shortcuts
- [ ] Navigation tips
- [ ] Common troubleshooting
- [ ] `--help` flag for CLI usage
- [ ] `--examples` flag for common use cases

---

## 4.5 Settings Screen

- [ ] Edit output directory
- [ ] Change default quality
- [ ] Toggle theme
- [ ] Adjust concurrent downloads
- [ ] Audio format/quality preferences
- [ ] Save changes to config file

---

## 4.6 Error Recovery

- [ ] Retry button on error screen
- [ ] Exponential backoff for retries
- [ ] Helpful error messages with solutions
- [ ] Specific suggestions per error type
- [ ] Log full error details for debugging

---

## 4.7 Desktop Notifications

- [ ] Notify on download complete (macOS)
- [ ] Notify on error (optional)
- [ ] Configurable in settings
- [ ] Cross-platform support (Linux, Windows)

---

## 4.8 Performance

- [ ] Efficient rendering (only redraw on changes)
- [ ] Throttle progress updates
- [ ] Lazy load history
- [ ] Minimize allocations in hot paths
- [ ] Profile and optimize bottlenecks

---

## Testing

- [ ] All themes render correctly
- [ ] Animations smooth at 60 FPS
- [ ] All shortcuts work
- [ ] Help accessible from all screens
- [ ] Settings persist
- [ ] No UI lag or stuttering

---

## Success Criteria

- [ ] Themes implemented and switchable
- [ ] Smooth animations throughout
- [ ] Complete keyboard shortcut system
- [ ] Comprehensive help system
- [ ] Settings screen functional
- [ ] Desktop notifications working
- [ ] Performance optimized

---

## Next Steps

Once Phase 4 is complete, proceed to [Phase 5: Distribution](phase-05-distribution.md)
