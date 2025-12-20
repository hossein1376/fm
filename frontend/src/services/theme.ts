/**
 * fm/frontend/src/services/theme.ts
 *
 * Theme service to manage light/dark theme state with persistence and system preference support.
 *
 * Usage:
 *  import themeService from '../services/theme'
 *  const { theme, setTheme, toggleTheme, followSystem, enableFollowSystem } = themeService
 *
 * The service exposes a reactive `theme` (readonly) that can be used in components,
 * and methods to change or toggle the theme. Theme is persisted to localStorage under key "fm_theme".
 *
 * It also supports optionally following the OS-level color scheme via `followSystem`.
 */

import { ref, readonly, watch } from "vue";

type Theme = "light" | "dark";

const STORAGE_KEY = "fm_theme";
const STORAGE_AUTO_KEY = "fm_theme_auto"; // whether to follow system preference

const isBrowser =
  typeof window !== "undefined" && typeof document !== "undefined";

function readStoredTheme(): Theme | null {
  if (!isBrowser) return null;
  try {
    const v = localStorage.getItem(STORAGE_KEY);
    if (v === "dark" || v === "light") return v;
    return null;
  } catch {
    return null;
  }
}

function writeStoredTheme(theme: Theme) {
  if (!isBrowser) return;
  try {
    localStorage.setItem(STORAGE_KEY, theme);
  } catch {
    // ignore storage errors
  }
}

function readStoredAuto(): boolean {
  if (!isBrowser) return false;
  try {
    const v = localStorage.getItem(STORAGE_AUTO_KEY);
    return v === "1" || v === "true";
  } catch {
    return false;
  }
}

function writeStoredAuto(value: boolean) {
  if (!isBrowser) return;
  try {
    localStorage.setItem(STORAGE_AUTO_KEY, value ? "1" : "0");
  } catch {
    // ignore
  }
}

function systemPrefersDark(): boolean {
  if (!isBrowser || !window.matchMedia) return false;
  try {
    return window.matchMedia("(prefers-color-scheme: dark)").matches;
  } catch {
    return false;
  }
}

function applyThemeToDocument(theme: Theme) {
  if (!isBrowser) return;
  try {
    // set an attribute so CSS can react to it
    document.documentElement.setAttribute("data-theme", theme);
    // also set a class for compatibility if needed
    document.documentElement.classList.remove("theme-light", "theme-dark");
    document.documentElement.classList.add(`theme-${theme}`);
  } catch {
    // ignore
  }
}

/**
 * Reactive state
 */
const _theme = ref<Theme>("light");
const _followSystem = ref<boolean>(false);

/**
 * Listen to system changes when followSystem is enabled
 */
let systemListener: ((e: MediaQueryListEvent) => void) | null = null;

function startSystemListener() {
  if (!isBrowser || !window.matchMedia) return;
  try {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    systemListener = (e: MediaQueryListEvent) => {
      const prefersDark = e.matches;
      // Only update theme if followSystem is true
      if (_followSystem.value) {
        const newTheme: Theme = prefersDark ? "dark" : "light";
        _theme.value = newTheme;
      }
    };
    // Modern browsers support addEventListener on MediaQueryList, but older use addListener
    if (typeof mq.addEventListener === "function") {
      mq.addEventListener("change", systemListener);
    } else if (typeof mq.addListener === "function") {
      // @ts-ignore - older API
      mq.addListener(systemListener);
    }
  } catch {
    // ignore errors
  }
}

function stopSystemListener() {
  if (!isBrowser || !window.matchMedia || !systemListener) return;
  try {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    if (typeof mq.removeEventListener === "function") {
      mq.removeEventListener("change", systemListener);
    } else if (typeof mq.removeListener === "function") {
      // @ts-ignore
      mq.removeListener(systemListener);
    }
  } catch {
    // ignore
  } finally {
    systemListener = null;
  }
}

/**
 * Initialize service state. Called automatically when module is evaluated,
 * but can be called again safely.
 */
function init() {
  // load follow-system preference
  const storedAuto = readStoredAuto();
  _followSystem.value = !!storedAuto;

  // determine initial theme
  const stored = readStoredTheme();
  if (_followSystem.value) {
    _theme.value = systemPrefersDark() ? "dark" : "light";
  } else if (stored) {
    _theme.value = stored;
  } else {
    // default: prefer system when available otherwise light
    _theme.value = systemPrefersDark() ? "dark" : "light";
  }

  applyThemeToDocument(_theme.value);

  // start listening if followSystem is enabled
  if (_followSystem.value) startSystemListener();
}

init();

/**
 * Watch and persist theme changes
 */
watch(
  _theme,
  (newVal) => {
    applyThemeToDocument(newVal);
    writeStoredTheme(newVal);
  },
  { immediate: false },
);

watch(
  _followSystem,
  (newVal) => {
    writeStoredAuto(newVal);
    if (newVal) {
      // adopt system immediately and start listening
      _theme.value = systemPrefersDark() ? "dark" : "light";
      startSystemListener();
    } else {
      // stop listening to system changes
      stopSystemListener();
    }
  },
  { immediate: false },
);

/**
 * Public API
 */
function setTheme(theme: Theme) {
  _followSystem.value = false; // explicit set disables followSystem
  _theme.value = theme;
}

function toggleTheme() {
  _followSystem.value = false;
  _theme.value = _theme.value === "dark" ? "light" : "dark";
}

function enableFollowSystem(enable = true) {
  _followSystem.value = enable;
  // followSystem watcher will do the rest (persist + adopt system preference)
}

function getTheme(): Theme {
  return _theme.value;
}

function isFollowingSystem(): boolean {
  return _followSystem.value;
}

/**
 * Export a simple singleton service.
 * Also export a composable-friendly helper to expose reactive values.
 */
const themeService = {
  // reactive refs (readonly wrappers)
  theme: readonly(_theme),
  followSystem: readonly(_followSystem),

  // methods
  init,
  setTheme,
  toggleTheme,
  enableFollowSystem,
  getTheme,
  isFollowingSystem,
};

export default themeService;

export {
  init as initThemeService,
  setTheme,
  toggleTheme,
  enableFollowSystem,
  getTheme,
  isFollowingSystem,
  themeService as ThemeService,
};

export type { Theme };
