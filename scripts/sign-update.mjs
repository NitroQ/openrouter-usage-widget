#!/usr/bin/env node
/**
 * Ed25519 signing tool for update installers.
 *
 * Usage:
 *   node scripts/sign-update.mjs keygen                          → generates keys/
 *   node scripts/sign-update.mjs sign <installer-path>           → produces <path>.sig
 *   node scripts/sign-update.mjs verify <installer> <sig-file>   → validates signature
 *   node scripts/sign-update.mjs info <installer-path>           → prints SHA-256
 */

import { generateKeyPairSync, createPrivateKey, createPublicKey, sign, verify, createHash } from "node:crypto";
import { readFileSync, writeFileSync, mkdirSync, existsSync } from "node:fs";

function keygen() {
  const { publicKey, privateKey } = generateKeyPairSync("ed25519");

  // Export raw bytes
  const pubRaw = publicKey.export({ type: "spki", format: "der" });
  const secRaw = privateKey.export({ type: "pkcs8", format: "der" });

  // Ed25519 SPKI: last 32 bytes = public key
  const pub32 = pubRaw.subarray(pubRaw.length - 32);
  // Ed25519 PKCS8: last 32 bytes = private seed
  const seed32 = secRaw.subarray(secRaw.length - 32);

  if (!existsSync("keys")) mkdirSync("keys", { recursive: true });
  writeFileSync("keys/update-signing.pub", pub32.toString("hex") + "\n");
  writeFileSync("keys/update-signing.sec", seed32.toString("hex") + "\n");
  writeFileSync("keys/update-signing-pub.der", pubRaw);
  writeFileSync("keys/update-signing-sec.der", secRaw);

  console.log("Generated keys/update-signing.pub  (public — embed in build)");
  console.log("Generated keys/update-signing.sec  (private — KEEP SECRET)");
  console.log(`\nPublic key hex: ${pub32.toString("hex")}`);
}

function getPrivateKey() {
  // Try env var first, then .der file, then .sec hex file
  if (process.env.UPDATE_PRIVATE_KEY_HEX) {
    const seed = Buffer.from(process.env.UPDATE_PRIVATE_KEY_HEX.trim(), "hex");
    return createPrivateKey({ key: seed, format: "ed25519" });
  }
  if (existsSync("keys/update-signing-sec.der")) {
    return createPrivateKey({ key: readFileSync("keys/update-signing-sec.der"), format: "der", type: "pkcs8" });
  }
  if (existsSync("keys/update-signing.sec")) {
    const seed = Buffer.from(readFileSync("keys/update-signing.sec", "utf8").trim(), "hex");
    return createPrivateKey({ key: seed, format: "ed25519" });
  }
  console.error("Error: No private key found. Run keygen first.");
  process.exit(1);
}

function getPublicKey() {
  if (process.env.UPDATE_PUBLIC_KEY_HEX) {
    const raw = Buffer.from(process.env.UPDATE_PUBLIC_KEY_HEX.trim(), "hex");
    return createPublicKey({ key: raw, format: "ed25519" });
  }
  if (existsSync("keys/update-signing-pub.der")) {
    return createPublicKey({ key: readFileSync("keys/update-signing-pub.der"), format: "der", type: "spki" });
  }
  if (existsSync("keys/update-signing.pub")) {
    const raw = Buffer.from(readFileSync("keys/update-signing.pub", "utf8").trim(), "hex");
    return createPublicKey({ key: raw, format: "ed25519" });
  }
  console.error("Error: No public key found. Run keygen first.");
  process.exit(1);
}

function signFile(filePath) {
  const privKey = getPrivateKey();
  const data = readFileSync(filePath);
  const sig = sign(null, data, privKey);
  const sigHex = sig.toString("hex");

  const sigPath = filePath + ".sig";
  writeFileSync(sigPath, sigHex + "\n");

  const sha256 = createHash("sha256").update(data).digest("hex");
  console.log(`Signed:  ${filePath}`);
  console.log(`Output:  ${sigPath} (${sigHex.length} hex chars)`);
  console.log(`SHA-256: ${sha256}`);
}

function verifyFile(filePath, sigPath) {
  const pubKey = getPublicKey();
  const data = readFileSync(filePath);
  const sigHex = readFileSync(sigPath, "utf8").trim();
  const sigBytes = Buffer.from(sigHex, "hex");

  const valid = verify(null, data, pubKey, sigBytes);
  console.log(valid ? "Signature VALID" : "Signature INVALID");
  process.exit(valid ? 0 : 1);
}

function printInfo(filePath) {
  const data = readFileSync(filePath);
  const sha256 = createHash("sha256").update(data).digest("hex");
  console.log(`File:    ${filePath}`);
  console.log(`Size:    ${data.length} bytes`);
  console.log(`SHA-256: ${sha256}`);
}

// --- Main ---
const [,, cmd, ...args] = process.argv;

if (cmd === "keygen") {
  keygen();
} else if (cmd === "sign") {
  if (!args[0]) { console.error("Usage: sign-update.mjs sign <installer-path>"); process.exit(1); }
  signFile(args[0]);
} else if (cmd === "verify") {
  if (!args[0] || !args[1]) { console.error("Usage: sign-update.mjs verify <installer> <sig-file>"); process.exit(1); }
  verifyFile(args[0], args[1]);
} else if (cmd === "info") {
  if (!args[0]) { console.error("Usage: sign-update.mjs info <installer-path>"); process.exit(1); }
  printInfo(args[0]);
} else {
  console.error("Usage: sign-update.mjs <keygen|sign|verify|info> [args]");
  process.exit(1);
}
