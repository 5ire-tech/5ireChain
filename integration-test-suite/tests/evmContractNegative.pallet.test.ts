import { expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import {
  killNodes,
  polkadotApi,
  polkadotApi as api,
  spawnNodes,
} from "../utils/util";
import { Keyring } from "@polkadot/api";
import { addressToEvm } from "@polkadot/util-crypto";
import { KeyringPair } from "@polkadot/keyring/types";
import { waitForEvent, waitNfinalizedBlocks } from "../utils/setup";
import { bytesToHex } from "web3-utils";
// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: "sr25519" });

const ERC20_BYTECODES = require("./contracts/MyToken.json").bytecode;

let aliceSubstrateAccount: KeyringPair;
let bobSubstrateAccount: KeyringPair;

let aliceEvmAccount: Uint8Array;
let bobEvmAccount: Uint8Array;

describe("EVM related tests", function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
    let accounts = await init();

    aliceSubstrateAccount = accounts.alice;
    bobSubstrateAccount = accounts.bob;
    aliceEvmAccount = accounts.aliceEthAccount;
    bobEvmAccount = accounts.bobEthAccount;

    //swap native to evm balance 10 coin
    const amount = api.createType("Balance", "10000000000000000000");
    const transaction = api.tx.evm.deposit(aliceEvmAccount, amount);

    const unsub = await transaction.signAndSend(
      aliceSubstrateAccount,
      (result) => {
        console.log(`Swap is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`Swap included at blockHash ${result.status.asInBlock}`);
          console.log(`Waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          console.log(`events are ${result.events}`);
          console.log(
            `Swap finalized at blockHash ${result.status.asFinalized}`
          );
          unsub();
        }
      }
    );

    await waitForEvent(api, "balances", "Transfer");
  });

  // Creation of contract should fail with invalid nonce
  it("Should fail with invalid nonce", async () => {
    await createContractWithInvalidNonceFailure(
      aliceEvmAccount,
      aliceSubstrateAccount
    );
  });

  // Creation of contract should fail with gas price too low
  it("Should fail with gas price too low", async () => {
    await createContractWithGasPriceTooLowFailure(
      aliceEvmAccount,
      aliceSubstrateAccount
    );
  });
  // Creation of contract should fail with gas limit too low
  it("Should fail with gas limit too low", async () => {
    await createContractWithGasLimitTooLowFailure(
      aliceEvmAccount,
      aliceSubstrateAccount
    );
  });
  // Creation of contract should fail with gas limit too high
  it("Should fail with gas limit too high", async () => {
    await createContractWithGasLimitTooHighFailure(
      aliceEvmAccount,
      aliceSubstrateAccount
    );
  });
  // Creation of contract should fail with balance low
  it("Should fail with balance low for evm", async () => {
    await createContractWithInsufficientBalanceFailure(
      bobEvmAccount,
      bobSubstrateAccount
    );
  });

  it("Execution of contracts transfer should fail with invalid nonce", async () => {
    const contract = await createContract(
      0,
      aliceEvmAccount,
      aliceSubstrateAccount
    );
    await transferTokenShouldFailWithNonceFailure(
      10,
      bytesToHex(Array.from(aliceEvmAccount)),
      bytesToHex(Array.from(bobEvmAccount)),
      bobSubstrateAccount,
      aliceSubstrateAccount,
      contract.address
    );
  });

  it("Execution of contracts transfer should fail with gas price too low failure", async () => {
    const contract = await createContract(
      1,
      aliceEvmAccount,
      aliceSubstrateAccount
    );
    await transferTokenShouldFailWithGasPriceTooLowFailure(
      2,
      bytesToHex(Array.from(aliceEvmAccount)),
      bytesToHex(Array.from(bobEvmAccount)),
      bobSubstrateAccount,
      aliceSubstrateAccount,
      contract.address
    );
  });

  it("Execution of contracts transfer should fail with gas limit too low failure", async () => {
    const contract = await createContract(
      2,
      aliceEvmAccount,
      aliceSubstrateAccount
    );
    await transferTokenShouldFailWithGasLimitTooLowFailure(
      3,
      bytesToHex(Array.from(aliceEvmAccount)),
      bytesToHex(Array.from(bobEvmAccount)),
      bobSubstrateAccount,
      aliceSubstrateAccount,
      contract.address
    );
  });

  after(async () => {
    await killNodes();
  });
});

// Setup the API and Alice Account
async function init() {
  const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
  const bob = keyring.addFromUri("//Bob", { name: "Bob default" });

  const aliceEthAccount = addressToEvm(alice.addressRaw);
  const bobEthAccount = addressToEvm(bob.addressRaw);
  return { alice, bob, aliceEthAccount, bobEthAccount };
}

// Create the ERC20 contract from ALICE
async function createContract(
  nonce: number,
  evmAddress: Uint8Array,
  alice: KeyringPair
) {
  console.log(`\n: Creating Smart Contract with nonce ${nonce}`);

  const source = evmAddress;
  const init = ERC20_BYTECODES;
  const value = 0;
  const gasLimit = 100_000_0;
  const maxFeePerGas = 100_000_000_0000;
  const maxPriorityFeePerGas: BigInt = BigInt(100_000_000);
  const accessList = null;

  const transaction = await api.tx.evm.create(
    source,
    init,
    value,
    gasLimit,
    maxFeePerGas,
    maxPriorityFeePerGas,
    nonce,
    accessList
  );

  const contract = new Promise<{ block: string; address: string }>(
    async (resolve, reject) => {
      const unsub = await transaction.signAndSend(
        alice,
        { tip: 200, nonce: -1 },
        (result) => {
          console.log(`Contract creation is ${result.status}`);
          if (result.status.isInBlock) {
            console.log(
              `Contract included at blockHash ${result.status.asInBlock}`
            );
            console.log(`Waiting for finalization... (can take a minute)`);
          } else if (result.status.isFinalized) {
            const data = JSON.stringify(result.events);
            console.log(data);

            const dataStr = JSON.parse(data);

            const filteredData = dataStr.filter(
              (item: any) => item.event.index === "0x3601"
            );
            const contractAddress = filteredData[0].event.data[0];
            expect(contractAddress).not.undefined;

            console.log(`Contract address: ${contractAddress}`);
            unsub();
            resolve({
              block: result.status.asFinalized.toString(),
              address: contractAddress,
            });
          }
        }
      );

      await waitForEvent(api, "evm", "Created");
    }
  );
  return contract;
}

// Transfer tokens to Bob should fail with Invalid Nonce
async function transferTokenShouldFailWithNonceFailure(
  nonce: number,
  aliceEthAccount: string,
  bobEthAccount: string,
  bob: KeyringPair,
  alice: KeyringPair,
  contractAddress: string
) {
  console.log(`Transfering Tokens to Bob EVM Account: ${bobEthAccount}`);

  console.log(`Preparing transfer of 0xdd`);
  const transferFnCode = `a9059cbb000000000000000000000000`;
  const tokensToTransfer = `00000000000000000000000000000000000000000000000000000000000000dd`;
  const inputCode = `0x${transferFnCode}${bobEthAccount.substring(
    2
  )}${tokensToTransfer}`;
  console.log(`Sending call input: ${inputCode}`);
  const gasLimit = 100_000_0;
  const maxFeePerGas = 100_000_000_0000;
  const maxPriorityFeePerGas: BigInt = BigInt(100_000_000);
  const accessList = null;
  const transaction = await api.tx.evm.call(
    aliceEthAccount,
    contractAddress,
    inputCode,
    0,
    gasLimit,
    maxFeePerGas,
    maxPriorityFeePerGas,
    nonce,
    accessList
  );

  const data = new Promise<{}>(async (resolve, reject) => {
    const unsub = await transaction.signAndSend(alice, (result) => {
      console.log(`Transfer is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(
          `Transfer included at blockHash ${result.status.asInBlock}`
        );
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        const data = JSON.stringify(result.events);
        const dataStr = JSON.parse(data);

        const filteredData = dataStr.filter(
          (item: any) => item.event.index === "0x0001"
        );
        console.log(`filteredData ${JSON.stringify(filteredData)}`);
        expect(filteredData[0].event.data[0].module.error == "0x05000000").true; //Invalid Nonce
        unsub();
        resolve({});
      }
    });
  });

  return data;
}

// Transfer tokens to Bob should fail with Gas Price too low Nonce
async function transferTokenShouldFailWithGasPriceTooLowFailure(
  nonce: number,
  aliceEthAccount: string,
  bobEthAccount: string,
  bob: KeyringPair,
  alice: KeyringPair,
  contractAddress: string
) {
  console.log(`Transfering Tokens to Bob EVM Account: ${bobEthAccount}`);

  console.log(`Preparing transfer of 0xdd`);
  const transferFnCode = `a9059cbb000000000000000000000000`;
  const tokensToTransfer = `00000000000000000000000000000000000000000000000000000000000000dd`;
  const inputCode = `0x${transferFnCode}${bobEthAccount.substring(
    2
  )}${tokensToTransfer}`;
  console.log(`Sending call input: ${inputCode}`);
  const gasLimit = polkadotApi.createType("Balance", "1000000");
  const maxFeePerGas = polkadotApi.createType("Balance", "100000000000000");
  const maxPriorityFeePerGas: BigInt = BigInt(100_000_000_000_000_000);
  const accessList = null;
  const transaction = await api.tx.evm.call(
    aliceEthAccount,
    contractAddress,
    inputCode,
    0,
    gasLimit,
    maxFeePerGas,
    maxPriorityFeePerGas,
    nonce,
    accessList
  );

  const data = new Promise<{}>(async (resolve, reject) => {
    const unsub = await transaction.signAndSend(alice, (result) => {
      console.log(`Transfer is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(
          `Transfer included at blockHash ${result.status.asInBlock}`
        );
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        const data = JSON.stringify(result.events);
        const dataStr = JSON.parse(data);

        const filteredData = dataStr.filter(
          (item: any) => item.event.index === "0x0001"
        );
        console.log(`filteredData ${JSON.stringify(filteredData)}`);
        expect(filteredData[0].event.data[0].module.error == "0x04000000").true; //Gas Price Too Low
        unsub();
        resolve({});
      }
    });
  });

  return data;
}

// Transfer tokens to Bob should fail with Gas Limit too low Nonce
async function transferTokenShouldFailWithGasLimitTooLowFailure(
  nonce: number,
  aliceEthAccount: string,
  bobEthAccount: string,
  bob: KeyringPair,
  alice: KeyringPair,
  contractAddress: string
) {
  console.log(`Transfering Tokens to Bob EVM Account: ${bobEthAccount}`);

  console.log(`Preparing transfer of 0xdd`);
  const transferFnCode = `a9059cbb000000000000000000000000`;
  const tokensToTransfer = `00000000000000000000000000000000000000000000000000000000000000dd`;
  const inputCode = `0x${transferFnCode}${bobEthAccount.substring(
    2
  )}${tokensToTransfer}`;
  console.log(`Sending call input: ${inputCode}`);
  const gasLimit = polkadotApi.createType("Balance", "1");
  const maxFeePerGas = polkadotApi.createType("Balance", "100000000000");
  const maxPriorityFeePerGas: BigInt = BigInt(100000000);
  const accessList = null;
  const transaction = await api.tx.evm.call(
    aliceEthAccount,
    contractAddress,
    inputCode,
    0,
    gasLimit,
    maxFeePerGas,
    maxPriorityFeePerGas,
    nonce,
    accessList
  );

  const data = new Promise<{}>(async (resolve, reject) => {
    const unsub = await transaction.signAndSend(alice, (result) => {
      console.log(`Transfer is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(
          `Transfer included at blockHash ${result.status.asInBlock}`
        );
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        const data = JSON.stringify(result.events);
        const dataStr = JSON.parse(data);

        const filteredData = dataStr.filter(
          (item: any) => item.event.index === "0x0001"
        );
        console.log(`filteredData ${JSON.stringify(filteredData)}`);
        expect(filteredData[0].event.data[0].module.error == "0x06000000").true; //Gas Limit Too Low
        unsub();
        resolve({});
      }
    });
  });

  return data;
}

async function createContractWithInvalidNonceFailure(
  evmAddress: any,
  alice: KeyringPair
) {
  console.log(`\n: Creating Smart Contract With Nonce Failure`);

  const source = evmAddress;
  const init = ERC20_BYTECODES;
  const value = 0;
  const gasLimit = 100_000;
  const maxFeePerGas = 100_000_000_000_000;
  const maxPriorityFeePerGas: BigInt = BigInt(100_000_000);
  const nonce = 1;
  const accessList = null;

  const transaction = await api.tx.evm.create(
    source,
    init,
    value,
    gasLimit,
    maxFeePerGas,
    maxPriorityFeePerGas,
    nonce,
    accessList
  );

  const contract = new Promise<{ block: string }>(async (resolve, reject) => {
    const unsub = await transaction.signAndSend(
      alice,
      { tip: 200, nonce: -1 },
      (result) => {
        console.log(`Contract creation is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(
            `Contract included at blockHash ${result.status.asInBlock}`
          );
          console.log(`Waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          const data = JSON.stringify(result.events);
          const dataStr = JSON.parse(data);
          const filteredData = dataStr.filter(
            (item: any) => item.event.index === "0x0001"
          );
          expect(filteredData[0].event.data[0].module.index == 54).true; //EVM
          expect(filteredData[0].event.data[0].module.error == "0x05000000")
            .true; //InvalidNonce, index 5

          unsub();
          resolve({
            block: result.status.asFinalized.toString(),
          });
        }
      }
    );
  });
  return contract;
}

async function createContractWithGasPriceTooLowFailure(
  evmAddress: any,
  alice: KeyringPair
) {
  const source = evmAddress;
  const init = ERC20_BYTECODES;
  const value = 0;
  const gasLimit = polkadotApi.createType("Balance", "10000000");
  const maxFeePerGas = polkadotApi.createType("Balance", "100000000000000");
  const maxPriorityFeePerGas: BigInt = BigInt(100_000_000_000_000_000);
  const nonce = 0;
  const accessList = null;

  const transaction = await api.tx.evm.create(
    source,
    init,
    value,
    gasLimit,
    maxFeePerGas,
    maxPriorityFeePerGas,
    nonce,
    accessList
  );

  const contract = new Promise<{ block: string }>(async (resolve, reject) => {
    const unsub = await transaction.signAndSend(
      alice,
      { tip: 200, nonce: -1 },
      (result) => {
        console.log(`Contract creation is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(
            `Contract included at blockHash ${result.status.asInBlock}`
          );
          console.log(`Waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          const data = JSON.stringify(result.events);
          const dataStr = JSON.parse(data);

          const filteredData = dataStr.filter(
            (item: any) => item.event.index === "0x0001"
          );
          console.log(`filteredData ${JSON.stringify(filteredData)}`);
          expect(filteredData[0].event.data[0].module.index == 54).true; //EVM
          expect(filteredData[0].event.data[0].module.error == "0x06000000")
            .true; //GasPriceTooLow

          unsub();
          resolve({
            block: result.status.asFinalized.toString(),
          });
        }
      }
    );
  });
}

async function createContractWithGasLimitTooLowFailure(
  evmAddress: any,
  alice: KeyringPair
) {
  const source = evmAddress;
  const init = ERC20_BYTECODES;
  const value = 0;
  const gasLimit = polkadotApi.createType("Balance", "1");
  const maxFeePerGas = polkadotApi.createType("Balance", "100000000000");
  const maxPriorityFeePerGas: BigInt = BigInt(100000000);
  const nonce = 0;
  const accessList = null;

  const transaction = await api.tx.evm.create(
    source,
    init,
    value,
    gasLimit,
    maxFeePerGas,
    maxPriorityFeePerGas,
    nonce,
    accessList
  );

  const contract = new Promise<{ block: string }>(async (resolve, reject) => {
    const unsub = await transaction.signAndSend(
      alice,
      { tip: 200, nonce: -1 },
      (result) => {
        console.log(`Contract creation is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(
            `Contract included at blockHash ${result.status.asInBlock}`
          );
          console.log(`Waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          const data = JSON.stringify(result.events);
          const dataStr = JSON.parse(data);

          const filteredData = dataStr.filter(
            (item: any) => item.event.index === "0x0001"
          );
          console.log(`filteredData ${JSON.stringify(filteredData)}`);
          expect(filteredData[0].event.data[0].module.index == 54).true; //EVM
          expect(filteredData[0].event.data[0].module.error == "0x06000000")
            .true; //GasLimitTooLow

          unsub();
          resolve({
            block: result.status.asFinalized.toString(),
          });
        }
      }
    );
  });
  return contract;
}

async function createContractWithGasLimitTooHighFailure(
  evmAddress: any,
  alice: KeyringPair
) {
  const source = evmAddress;
  const init = ERC20_BYTECODES;
  const value = 0;
  const gasLimit = polkadotApi.createType("Balance", "15000000");
  const maxFeePerGas = polkadotApi.createType("Balance", "10000000000000");
  const maxPriorityFeePerGas: BigInt = BigInt(1000000000000);
  const nonce = 0;
  const accessList = null;

  const transaction = await api.tx.evm.create(
    source,
    init,
    value,
    gasLimit,
    maxFeePerGas,
    maxPriorityFeePerGas,
    nonce,
    accessList
  );

  const contract = new Promise<{ block: string }>(async (resolve, reject) => {
    try {
      const unsub = await transaction.signAndSend(
        alice,
        { tip: 200, nonce: -1 },
        (result) => {
          console.log(`Contract creation is ${result.status}`);
          if (result.status.isInBlock) {
            console.log(
              `Contract included at blockHash ${result.status.asInBlock}`
            );
            console.log(`Waiting for finalization... (can take a minute)`);
          } else if (result.status.isFinalized) {
            const data = JSON.stringify(result.events);
            const dataStr = JSON.parse(data);

            const filteredData = dataStr.filter(
              (item: any) => item.event.index === "0x0001"
            );
            console.log(`filteredData ${JSON.stringify(filteredData)}`);
            expect(filteredData[0].event.data[0].module.index == 55).true; //EVM
            expect(filteredData[0].event.data[0].module.error == "0x06000000")
              .true; //GasLimitTooLow

            unsub();
            resolve({
              block: result.status.asFinalized.toString(),
            });
          }
        }
      );
    } catch (error) {
      reject(error);
    }
  });
  expect(contract).to.throw;
  //return contract;
}

async function createContractWithInsufficientBalanceFailure(
  evmAddress: any,
  alice: KeyringPair
) {
  const source = evmAddress;
  const init = ERC20_BYTECODES;
  const value = 0;
  const gasLimit = polkadotApi.createType("Balance", "10000000");
  const maxFeePerGas = polkadotApi.createType("Balance", "10000000000000");
  const maxPriorityFeePerGas: BigInt = BigInt(1000000000000);
  const nonce = 0;
  const accessList = null;

  const transaction = await api.tx.evm.create(
    source,
    init,
    value,
    gasLimit,
    maxFeePerGas,
    maxPriorityFeePerGas,
    nonce,
    accessList
  );

  const contract = new Promise<{ block: string }>(async (resolve, reject) => {
    try {
      const unsub = await transaction.signAndSend(
        alice,
        { tip: 200, nonce: -1 },
        (result) => {
          console.log(`Contract creation is ${result.status}`);
          if (result.status.isInBlock) {
            console.log(
              `Contract included at blockHash ${result.status.asInBlock}`
            );
            console.log(`Waiting for finalization... (can take a minute)`);
          } else if (result.status.isFinalized) {
            const data = JSON.stringify(result.events);
            const dataStr = JSON.parse(data);

            const filteredData = dataStr.filter(
              (item: any) => item.event.index === "0x0001"
            );
            console.log(`filteredData ${JSON.stringify(filteredData)}`);
            expect(filteredData[0].event.data[0].module.index == 6).true; //Balance
            expect(filteredData[0].event.data[0].module.error == "0x02000000")
              .true; //InsufficientBalance

            unsub();
            resolve({
              block: result.status.asFinalized.toString(),
            });
          }
        }
      );
    } catch (error) {
      reject(error);
    }
  });
  expect(contract).to.throw;
}
