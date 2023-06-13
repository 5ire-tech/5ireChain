import { expect } from 'chai';
import { BLOCK_TIME } from '../utils/constants';
import {killNodes, polkadotApi, polkadotApi as api, spawnNodes} from "../utils/util";
import {Keyring, WsProvider} from "@polkadot/api";
import {addressToEvm} from "@polkadot/util-crypto";
import { KeyringPair } from '@polkadot/keyring/types';
import {waitForEvent} from "../utils/setup";
// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: 'sr25519' });

const ERC20_BYTECODES = require("./contracts/MyToken.json").bytecode;

describe('EVM related tests', function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should init and create contracts
  it('Should init and create contracts', async () => {
    const {alice, aliceEthAccount} = await init();

    await createContract(aliceEthAccount, alice)
  });

  // Should do evm deposit
  it('Should do evm deposit', async () => {
    const {alice, bob, aliceEthAccount} = await init();
    await depositInEVM(aliceEthAccount, alice, bob);
  });

  after(async () => {
    //await killNodes();
  });

  // Setup the API and Alice Account
  async function init() {
    const alice = keyring.addFromUri('//Alice', {name: 'Alice default'});
    const bob = keyring.addFromUri('//Bob', {name: 'Bob default'});

    const aliceEthAccount = addressToEvm(alice.addressRaw);
    return {alice, bob, aliceEthAccount};
  }

  // Create the ERC20 contract from ALICE
  async function createContract(evmAddress: any, alice: KeyringPair) {

    console.log(`\n: Creating Smart Contract`);

    const source = evmAddress;
    const init = ERC20_BYTECODES;
    const value = 0;
    const gasLimit = 100_000_00;
    const maxFeePerGas = 100_000_000_000;
    const maxPriorityFeePerGas: BigInt = BigInt(100_000_000);
    const nonce = 0;
    const accessList = null;

    const transaction = await api.tx.evm.create(source, init, value, gasLimit, maxFeePerGas, maxPriorityFeePerGas, nonce, accessList);

    const contract = new Promise<{ block: string, address: string }>(async (resolve, reject) => {
      const unsub = await transaction.signAndSend(alice, {tip: 2000, nonce: -1}, (result) => {
        console.log(`Contract creation is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`Contract included at blockHash ${result.status.asInBlock}`);
          console.log(`Waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          const data = JSON.stringify(result.events);
          console.log(data);

          const dataStr = JSON.parse(data);

          const filteredData = dataStr.filter((item: any) => item.event.index === "0x3a01");
          const contractAddress = filteredData[0].event.data[0];
          expect(contractAddress).not.undefined;

          console.log(`Contract address: ${contractAddress}`);
          unsub();
          resolve({
            block: result.status.asFinalized.toString(),
            address: contractAddress
          });
        }
      });

      await waitForEvent(api, 'evm', 'Created')

    });
    return contract;
  }

  // Deposit into evm
  async function depositInEVM(aliceEthAccount: Uint8Array, alice: KeyringPair, bob: KeyringPair) {
    const address = aliceEthAccount;
    const value = 700000;

    const deposit = await api.tx.evm.deposit(address, value);

    const transaction = new Promise<{}>(async (resolve, reject) => {
      const unsub = await deposit.signAndSend(bob, {tip: 2000, nonce: -1}, (result) => {
        console.log(`EVM Deposit is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`EVM Deposit included at blockHash ${result.status.asInBlock}`);
          console.log(`Waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          const data = JSON.stringify(result.events);
          console.log(data);

          const dataStr = JSON.parse(data);

          unsub();
          resolve({});
        }
      });

      // Retrieve the account balance & nonce for Bob
      // @ts-ignore
      const {nonce: bobNonce, data: bobBalance} = await polkadotApi.query.system.account(alice.address);
      console.log(`alice balance is ${bobBalance.free.toHuman()}`);
      //await waitForEvent(api, 'evm', 'Created')
    });
  }
});
