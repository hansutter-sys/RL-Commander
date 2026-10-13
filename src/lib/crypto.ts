// Web Crypto API helper for PBKDF2 + AES-GCM encryption & SHA-256 PIN hashing

const DEFAULT_SALT = new TextEncoder().encode("rl_commander_static_salt_v1");
const DEFAULT_KEY_SECRET = "rl_commander_internal_vault_key_2026";

export async function hashPin(pin: string): Promise<string> {
  const enc = new TextEncoder();
  const data = enc.encode(pin + "_rlcommander_pin_salt_v1");
  const hashBuffer = await crypto.subtle.digest("SHA-256", data);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map((b) => b.toString(16).padStart(2, "0")).join("");
}

export async function verifyPin(pin: string, storedHash: string): Promise<boolean> {
  if (!storedHash) return false;
  const computed = await hashPin(pin);
  return computed === storedHash;
}

async function deriveAesKey(secret: string, salt: Uint8Array): Promise<CryptoKey> {
  const enc = new TextEncoder();
  const keyMaterial = await crypto.subtle.importKey(
    "raw",
    enc.encode(secret),
    "PBKDF2",
    false,
    ["deriveKey"]
  );
  return crypto.subtle.deriveKey(
    {
      name: "PBKDF2",
      salt: salt as BufferSource,
      iterations: 100000,
      hash: "SHA-256",
    },
    keyMaterial,
    { name: "AES-GCM", length: 256 },
    false,
    ["encrypt", "decrypt"]
  );
}

export async function encryptData(plainText: string, secretKey?: string): Promise<string> {
  const secret = secretKey || DEFAULT_KEY_SECRET;
  const salt = crypto.getRandomValues(new Uint8Array(16));
  const iv = crypto.getRandomValues(new Uint8Array(12));
  const key = await deriveAesKey(secret, salt);
  const enc = new TextEncoder();

  const encrypted = await crypto.subtle.encrypt(
    { name: "AES-GCM", iv },
    key,
    enc.encode(plainText)
  );

  const combined = new Uint8Array(salt.length + iv.length + encrypted.byteLength);
  combined.set(salt, 0);
  combined.set(iv, salt.length);
  combined.set(new Uint8Array(encrypted), salt.length + iv.length);

  return btoa(String.fromCharCode(...combined));
}

export async function decryptData(cipherBase64: string, secretKey?: string): Promise<string> {
  const secret = secretKey || DEFAULT_KEY_SECRET;
  const raw = Uint8Array.from(atob(cipherBase64), (c) => c.charCodeAt(0));
  const salt = raw.slice(0, 16);
  const iv = raw.slice(16, 28);
  const data = raw.slice(28);

  const key = await deriveAesKey(secret, salt);
  const decrypted = await crypto.subtle.decrypt(
    { name: "AES-GCM", iv },
    key,
    data
  );
  return new TextDecoder().decode(decrypted);
}
