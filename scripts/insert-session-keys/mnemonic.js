const { ethers } = require('ethers');

// Generate a random secret seed (32 bytes)
const secretSeed = "0xc8e2d366c7d28161b194f225d0278120812319c329fbfc9043ea517e0b2da129";

// Convert the secret seed to a mnemonic phrase
const mnemonic = ethers.utils.entropyToMnemonic(secretSeed);

console.log("Secret Seed:", secretSeed);
console.log("Mnemonic Phrase:", mnemonic);
