import { writable } from "svelte/store";

export interface Settings {
  showDiskSpace: boolean;
  showDriveButtons: boolean;
  defaultLeftPath: string;
  defaultRightPath: string;
  confirmOnDelete: boolean;
}

const DEFAULT_SETTINGS: Settings = {
  showDiskSpace: true,
  showDriveButtons: true,
  defaultLeftPath: "/",
  defaultRightPath: "/home",
  confirmOnDelete: true,
};

const STORAGE_KEY = "rl_commander_settings";

function loadSettings(): Settings {
  if (typeof localStorage !== "undefined") {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored) {
        return { ...DEFAULT_SETTINGS, ...JSON.parse(stored) };
      }
    } catch (e) {
      console.error("Failed to load settings from localStorage:", e);
    }
  }
  return DEFAULT_SETTINGS;
}

function createSettingsStore() {
  const { subscribe, set, update } = writable<Settings>(loadSettings());

  return {
    subscribe,
    set: (value: Settings) => {
      if (typeof localStorage !== "undefined") {
        try {
          localStorage.setItem(STORAGE_KEY, JSON.stringify(value));
        } catch (e) {
          console.error("Failed to save settings to localStorage:", e);
        }
      }
      set(value);
    },
    update: (fn: (settings: Settings) => Settings) => {
      update((current) => {
        const updated = fn(current);
        if (typeof localStorage !== "undefined") {
          try {
            localStorage.setItem(STORAGE_KEY, JSON.stringify(updated));
          } catch (e) {
            console.error("Failed to save settings to localStorage:", e);
          }
        }
        return updated;
      });
    },
    reset: () => {
      if (typeof localStorage !== "undefined") {
        try {
          localStorage.removeItem(STORAGE_KEY);
        } catch (e) {}
      }
      set(DEFAULT_SETTINGS);
    },
  };
}

export const settings = createSettingsStore();
