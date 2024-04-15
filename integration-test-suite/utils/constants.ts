export const SECONDS = 1000;
export const MINUTES = 60 * SECONDS;
export const BLOCK_TIME = 3 * SECONDS;
export const ACC1_PK =
  "0x0000000000000000000000000000000000000000000000000000000000000001";
export const ACC2_PK =
  "0x0000000000000000000000000000000000000000000000000000000000000002";

export const ETH_BLOCK_GAS_LIMIT = 75000000; // The same configuration as runtime
export const ETH_BLOCK_POV_LIMIT = 5 * 1024 * 1024; // The same configuration as runtime

export const GENESIS_ACCOUNT_BALANCE = "19342813113834066795298815";
export const EXISTENTIAL_DEPOSIT = 0; // The minimum amount required to keep an account open

export const GENESIS_ACCOUNTS: string[] = [
  "0x48Df7B35247786418a7e279e508325952B9Fc92F",
  "0x74E4214c9C3D9726E1A0B57357C4dd117641c536",
  "0xFE31f14425993A3d9aeDEd195C56999eBE097d92",
]; //Genesis account addresses

export const CHAIN_ID = 997;
