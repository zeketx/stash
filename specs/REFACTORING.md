# Architecture Improvements

**Current Grade:** B+ (85/100) - Good foundation, some issues to clean up

---

## 🔴 HIGH PRIORITY (Before v1.0)

### 1. Fix Duplicate VideoInfo Types
**Time:** 2-4 hours

**Problem:** Two different `VideoInfo` types with same name
- `downloader.rs` - Full yt-dlp metadata  
- `ui/app.rs` - Display-optimized data

**Fix:** Rename one to `VideoMetadata`, add `to_display_info()` conversion

---

### 2. Extract Commands from main.rs
**Time:** 4-8 hours

**Problem:** main.rs is 509 lines, handles too much
- CLI parsing, config, downloads, subcommands all mixed

**Fix:** Create `src/commands/` directory, one module per command

---

### 3. Remove Demo Code
**Time:** 2-3 hours

**Problem:** TUI has `simulate_download()` and `simulate_fetch()` 
- Production code shouldn't have fake implementations

**Fix:** Connect to real `Downloader`, add progress callbacks, or feature-gate

---

## 🟡 MEDIUM PRIORITY (v1.1)

### 4. Add Domain Layer
**Time:** 8-16 hours (or skip for simpler approach)

**Problem:** Business logic scattered, no clear separation
- Hard to tell where core logic lives

**Fix:** Create `src/domain/` for business logic, `src/infrastructure/` for external deps

**Alternative:** Just create `src/core/` and `src/infra/` for 80% benefit

---

### 5. Reorganize Folders
**Time:** 4-6 hours

**Problem:** Everything at root level, no layering visible

**Fix:** 
```
src/
├── domain/      (or core/)
├── cli/
├── tui/         (rename from ui/)
├── infrastructure/
└── shared/
```

---

## 🟢 LOW PRIORITY (Polish)

### 6. Welcome Screen Animations
**Time:** 3-6 hours

Add color-cycling banner, pulsing menu selection

---

### 7. Extract Constants
**Time:** 1-2 hours

Magic numbers → `src/constants.rs`

---

### 8. Integration Tests
**Time:** 4-8 hours

Add `tests/` directory with CLI/TUI/E2E tests

---

## Summary

**Total effort:** 28-53 hours across 8 improvements

**Recommended order:**
1. Do HIGH priority (clean foundation)
2. Pick 1-2 MEDIUM (architecture polish)  
3. Add LOW priority as desired (nice-to-haves)

**Not over-engineered for feature scope** - this is a feature-rich app (15+ features), complexity is appropriate
