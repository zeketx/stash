# Welcome Screen Redesign: Action-First UX

**Goal:** Eliminate the intermediate welcome screen and land users directly in an input-ready state

**Effort:** ~90-110 lines across 5 files | 2-3 hours

**Status:** 📋 Planned

---

## Problem

Current flow requires unnecessary steps:
```
Welcome Screen → Press any key → URL Input Screen → Type URL
```

This adds friction and doesn't respect the user's time.

---

## Solution

**Merge Welcome + URL Input into a single, action-ready screen**

```
Input-Ready Screen → Type/paste URL immediately
```

### Visual Design

```
┌─────────────────────────────────────────────┐
│  What would you like to download today?    │  ← Conversational
│                                             │
│  ┌─────────────────────────────────────┐   │
│  │ https://youtube.com/watch?v=...▊   │   │  ← Active cursor
│  └─────────────────────────────────────┘   │
│  Press Enter to continue or paste to start │  ← Hint
│                                             │
│  ╔═══════════════════════════════════╗     │
│  ║ Recent Downloads                   ║     │
│  ║ • Video Title    3 mins ago        ║     │
│  ╚═══════════════════════════════════╝     │
│                                             │
│  [S] Settings  [H] Help  [Q] Quit          │  ← Subdued footer
└─────────────────────────────────────────────┘
```

---

## Implementation

### 1. Update App State (`src/tui/app.rs`)
- Remove `Welcome` variant from `AppState` enum
- Start with `UrlInput` state immediately in `App::new()`
- Delete `go_to_url_input()` method

### 2. Redesign Input Screen (`src/tui/screens/url_input.rs`)
- Add conversational greeting: *"What would you like to download today?"*
- Update placeholder: *"Paste a YouTube URL or press Ctrl+V"*
- Add hint text: *"Press Enter to continue or paste a URL to start"*
- Update footer: `[Enter] Continue [Ctrl+U] Clear │ [S] Settings [H] Help [Q] Quit`
- Keep recent downloads section unchanged

### 3. Update Event Handler (`src/tui/runner.rs`)
- Remove `AppState::Welcome` match arm
- Add S/H/Q key handlers to `UrlInput` state:
  - `S` → go_to_settings()
  - `H` → go_to_help()
  - `Q` → exit

### 4. Clean Up (`src/tui/screens/`)
- Delete `welcome.rs`
- Remove welcome imports from `mod.rs`
- Keep banner widget (potential future use)

---

## Testing

```bash
cargo build --release && ./target/release/ytdl -i

# ✅ Cursor visible immediately on launch
# ✅ Type/paste URL works seamlessly
# ✅ S/H/Q keys accessible from main screen
# ✅ All progress screens unchanged (fetching, downloading, etc.)
```

---

## Design Rationale

**Why remove the banner?**
- Modern TUI tools (Spotlight, Raycast) launch directly into action
- Banner adds visual noise when users have one goal: paste URL
- Saves keystrokes and time

**Why conversational tone?**
- "What would you like to download?" feels friendly, not robotic
- Matches modern terminal UX trends (lazygit, gh, glow)

**Why footer for helpers?**
- Standard TUI pattern (visible but not dominant)
- Maximizes vertical space for content
- Secondary actions should support, not distract

---

## Files Changed

| File | Action | Impact |
|------|--------|--------|
| `src/tui/app.rs` | Modify | 10-15 lines |
| `src/tui/screens/url_input.rs` | Modify | 50-60 lines |
| `src/tui/runner.rs` | Modify | 20-30 lines |
| `src/tui/screens/welcome.rs` | Delete | - |
| `src/tui/screens/mod.rs` | Modify | 2 lines |

---

## Outcome

- ✅ Zero-friction launch experience
- ✅ Immediate action readiness
- ✅ Conversational, friendly tone
- ✅ All progress/loading UI preserved
- ✅ Settings/Help accessible but subtle

**Result:** Transform ytdl from menu-driven to action-first.
