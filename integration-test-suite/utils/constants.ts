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

export const TEST_ACCOUNT = "0xdd33Af49c851553841E94066B54Fd28612522901";
export const TEST_ACCOUNT_PRIVATE_KEY =
  "0x4ca933bffe83185dda76e7913fc96e5c97cdb7ca1fbfcc085d6376e6f564ef71";
export const TEST_CONTRACT_ADDRESS = "0xc2bf5f29a4384b1ab0c063e1c666f02121b6084a";
export const GENESIS_ACCOUNT_0_PRIVATE_KEY = "0xc52db56e56fb6e827add1192dd0d78d336e0d41f8bcc481784486372759c9f46";
export const INVALID_OPCODE_BYTECODE = "0x6080604052348015600e575f80fd5b5060d280601a5f395ff3fe6080604052348015600e575f80fd5b50600436106030575f3560e01c806328b5e32b1460345780638381f58a14603c575b5f80fd5b603a6056565b005b6042606a565b604051604d91906085565b60405180910390f35b60015f81905550619c405a1015606857fe5b565b5f5481565b5f819050919050565b607f81606f565b82525050565b5f60208201905060965f8301846078565b9291505056fea2646970667358221220c90ea075cc08a2f562e56a5efb0b01c9c98c36541bf26b5f5b11d2336ceb3b0864736f6c63430008190033";
