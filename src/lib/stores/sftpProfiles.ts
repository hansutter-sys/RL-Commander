import { writable, get } from "svelte/store";
import type { SftpProfile, SftpConfig } from "../types";
import { encryptData, decryptData, verifyPin } from "../crypto";
import { settings } from "./settings";
import { showToast } from "./commander";

const STORAGE_KEY = "rl_commander_sftp_profiles_vault_v1";

export const sftpProfiles = writable<SftpProfile[]>([]);
export const isSftpUnlocked = writable<boolean>(false);
export const cachedPin = writable<string>("");

// Default template profiles if empty
const DEFAULT_PROFILES: SftpProfile[] = [
  {
    id: "demo_local_sftp",
    name: "Lokal SFTP / SSH Test",
    host: "127.0.0.1",
    port: 22,
    username: "hans",
    password: "",
    private_key_path: "",
    created_at: Date.now(),
  },
];

export async function initSftpProfiles(): Promise<void> {
  const currentSettings = get(settings);

  if (currentSettings.pinProtectionEnabled) {
    // Requires PIN unlock first
    isSftpUnlocked.set(false);
    sftpProfiles.set([]);
  } else {
    // PIN protection disabled: automatically load using default secret
    try {
      await loadProfilesFromStorage();
      isSftpUnlocked.set(true);
    } catch (e) {
      console.warn("Failed to load SFTP profiles:", e);
      sftpProfiles.set(DEFAULT_PROFILES);
      isSftpUnlocked.set(true);
    }
  }
}

export async function unlockWithPin(pin: string): Promise<boolean> {
  const currentSettings = get(settings);
  if (!currentSettings.pinProtectionEnabled) {
    isSftpUnlocked.set(true);
    return true;
  }

  const isValid = await verifyPin(pin, currentSettings.pinHash);
  if (!isValid) {
    return false;
  }

  cachedPin.set(pin);
  try {
    await loadProfilesFromStorage(pin);
  } catch (err) {
    console.warn("Could not decrypt profiles with PIN, initializing default:", err);
    sftpProfiles.set(DEFAULT_PROFILES);
  }
  isSftpUnlocked.set(true);
  return true;
}

export function lockSftpProfiles(): void {
  const currentSettings = get(settings);
  if (currentSettings.pinProtectionEnabled) {
    isSftpUnlocked.set(false);
    cachedPin.set("");
    sftpProfiles.set([]);
  }
}

async function loadProfilesFromStorage(pinKey?: string): Promise<void> {
  if (typeof localStorage === "undefined") return;

  const rawEncrypted = localStorage.getItem(STORAGE_KEY);
  if (!rawEncrypted) {
    sftpProfiles.set(DEFAULT_PROFILES);
    await saveProfilesToStorage(DEFAULT_PROFILES, pinKey);
    return;
  }

  try {
    const jsonStr = await decryptData(rawEncrypted, pinKey);
    const parsed: SftpProfile[] = JSON.parse(jsonStr);
    sftpProfiles.set(parsed);
  } catch (err) {
    console.error("Decryption failed:", err);
    throw err;
  }
}

export async function saveProfilesToStorage(
  profiles: SftpProfile[],
  pinKey?: string
): Promise<void> {
  if (typeof localStorage === "undefined") return;

  const currentSettings = get(settings);
  const key = currentSettings.pinProtectionEnabled ? (pinKey || get(cachedPin)) : undefined;

  try {
    const jsonStr = JSON.stringify(profiles);
    const encrypted = await encryptData(jsonStr, key);
    localStorage.setItem(STORAGE_KEY, encrypted);
  } catch (err) {
    console.error("Encryption failed:", err);
  }
}

export async function addOrUpdateSftpProfile(profile: SftpProfile): Promise<void> {
  sftpProfiles.update((list) => {
    const index = list.findIndex((p) => p.id === profile.id);
    let updated: SftpProfile[];
    if (index >= 0) {
      updated = [...list];
      updated[index] = profile;
    } else {
      updated = [...list, profile];
    }
    saveProfilesToStorage(updated);
    return updated;
  });
  showToast(`Sparade serverprofil: ${profile.name}`);
}

export async function deleteSftpProfile(id: string): Promise<void> {
  sftpProfiles.update((list) => {
    const item = list.find((p) => p.id === id);
    const updated = list.filter((p) => p.id !== id);
    saveProfilesToStorage(updated);
    if (item) {
      showToast(`Tog bort serverprofil: ${item.name}`);
    }
    return updated;
  });
}

export async function reEncryptVaultWithNewPin(
  newPin: string | null,
  currentPin?: string
): Promise<void> {
  let currentList = get(sftpProfiles);

  // If profiles not loaded yet, try to load first
  if (currentList.length === 0 && typeof localStorage !== "undefined") {
    const rawEncrypted = localStorage.getItem(STORAGE_KEY);
    if (rawEncrypted) {
      try {
        const json = await decryptData(rawEncrypted, currentPin);
        currentList = JSON.parse(json);
        sftpProfiles.set(currentList);
      } catch (e) {
        console.warn("Could not decrypt existing vault during PIN change:", e);
      }
    }
  }

  if (newPin) {
    cachedPin.set(newPin);
    await saveProfilesToStorage(currentList, newPin);
    isSftpUnlocked.set(true);
  } else {
    // PIN disabled: save with default key
    cachedPin.set("");
    await saveProfilesToStorage(currentList, undefined);
    isSftpUnlocked.set(true);
  }
}

export function profileToSftpConfig(profile: SftpProfile): SftpConfig {
  return {
    host: profile.host,
    port: profile.port || 22,
    username: profile.username,
    password: profile.password || undefined,
    private_key_path: profile.private_key_path || undefined,
  };
}
