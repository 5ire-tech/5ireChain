import type { Codec } from '@polkadot/types-codec/types';
import { ApiPromise, Keyring } from '@polkadot/api';
import { SubmittableExtrinsic } from '@polkadot/api/types';
import child from 'child_process';
import { ECPair } from 'ecpair';
import { ethers } from 'ethers';
import {mnemonicGenerate} from "@polkadot/util-crypto";
import {BN} from "@polkadot/util";
import {WeightV2} from "@polkadot/types/interfaces";
import {DetectCodec} from "@polkadot/types/types/detect";

export const endpoint = 'ws://127.0.0.1:9944';
export const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

export async function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * Converts hex string to bytes
 */
export const hexToBytes = function (hex: any) {
  for (var bytes = [], c = 0; c < hex.length; c += 2) {
    bytes.push(parseInt(hex.substr(c, 2), 16));
  }
  return bytes;
};

/**
 * Listens for a block
 */
export const listenOneBlock = async function (api: ApiPromise) {
  const unsubscribe = await api.rpc.chain.subscribeNewHeads((header) => {
    console.log(`Chain is at block: #${header.hash}`);
    unsubscribe();
  });
};

/**
 * Wait after a number of finalized block
 */
export const waitNfinalizedBlocks = async function (
  api: ApiPromise,
  n: number,
  timeout: number
) {
  return new Promise<void>(async (resolve, _reject) => {
    let count = 0;
    const unsubscribe = await api.rpc.chain.subscribeNewHeads((header) => {
      count++;
      if (count == n) {
        unsubscribe();
        resolve();
      }
      setTimeout(() => {
        unsubscribe();
        resolve();
      }, timeout * 1000);
    });
  });
};

/**
 * @description: fast forward {n} blocks from the current block number.
 */
export async function fastForward(
  api: ApiPromise,
  n: number,
  { delayBetweenBlocks }: { delayBetweenBlocks?: number } = {
    delayBetweenBlocks: 5,
  }
): Promise<void> {
  for (let i = 0; i < n; i++) {
    const createEmpty = true;
    const finalize = true;
    await api.rpc.engine.createBlock(createEmpty, finalize);
    // sleep for delayBetweenBlocks milliseconds
    await new Promise((resolve) => setTimeout(resolve, delayBetweenBlocks));
  }
}

/**
 * Fast forwards a block
 */
export async function fastForwardTo(
  api: ApiPromise,
  blockNumber: number,
  { delayBetweenBlocks }: { delayBetweenBlocks?: number } = {
    delayBetweenBlocks: 0,
  }
): Promise<void> {
  const currentBlockNumber = await api.rpc.chain.getHeader();
  const diff = blockNumber - currentBlockNumber.number.toNumber();
  if (diff > 0) {
    await fastForward(api, diff, { delayBetweenBlocks });
  }
}

/**
 * Prints the list of validators
 */
export const printValidators = async function (api: ApiPromise) {
  const [accountNonce, now, validators] = await Promise.all([
    api.query.system.account(ALICE).then((account) => api.registry.createType(`number`, account.toU8a())),
    api.query.timestamp.now(),
    api.query.session.validators().then((account) => api.registry.createType(`Vec<Address>`, account.toU8a())),
  ]);

  console.log(`accountNonce(${ALICE}) ${accountNonce}`);
  console.log(`last block timestamp ${now.toHuman()}`);

  if (validators && validators.length > 0) {
    const validatorBalances = await Promise.all(
      validators.map((authorityId) => api.query.system.account(authorityId))
    );

    console.log(
      'validators',
      validators.map((authorityId, index) => {
        const balance = validatorBalances[index].toJSON();
        ({
          address: authorityId.toString(),
          balance: balance?["data"]?["free"]:0:0,
          nonce: balance?["nonce"]:0,
        })
      })
    );
  }
};

// a global variable to check if the node is already running or not.
// to avoid running multiple nodes with the same authority at the same time.
const __NODE_STATE: {
  [authorityId: string]: {
    process: child.ChildProcess | null;
    isRunning: boolean;
  };
} = {
  alice: { isRunning: false, process: null },
  bob: { isRunning: false, process: null },
  charlie: { isRunning: false, process: null },
  dave: { isRunning: false, process: null },
  eve: { isRunning: false, process: null },
  ferdie: { isRunning: false, process: null },
};

type StartOption = {
  tmp: boolean;
  printLogs: boolean;
  chain?: 'dev' | 'local';
};

const defaultOptions: StartOption = {
  tmp: true,
  printLogs: false,
  chain: 'local',
};

/**
 * Start 5ire chain node with different authorities and ports
 */
export function start5ireChainNode(
  authority: 'alice' | 'bob' | 'charlie' | 'dave' | 'eve' | 'ferdie',
  options: StartOption = defaultOptions
): child.ChildProcess {
  options.chain ??= 'local';

  if (__NODE_STATE[authority].isRunning) {
    return __NODE_STATE[authority].process!;
  }
  const gitRoot = child
    .execSync('git rev-parse --show-toplevel')
    .toString()
    .trim();
  const nodePath = `${gitRoot}/target/release/firechain-node`;
  const ports = {
    alice: { ws: 9944, http: 9933, p2p: 30333 },
    bob: { ws: 9945, http: 9934, p2p: 30334 },
    charlie: { ws: 9946, http: 9935, p2p: 30335 },
    dave: { ws: 9947, http: 9936, p2p: 30336 },
    eve: { ws: 9948, http: 9937, p2p: 30337 },
    ferdie: { ws: 9949, http: 9938, p2p: 30338 },
  };
  const proc = child.spawn(
    nodePath,
    [
      `--${authority}`,
      options.printLogs ? '-linfo' : '-lerror',
      `--chain`,
      `qa-dev`,
      `--tmp`,
      `--rpc-port=${ports[authority].ws}`,
      `--port=${ports[authority].p2p}`,
      ...(authority == 'alice'
        ? [
          '--node-key',
          '0000000000000000000000000000000000000000000000000000000000000001',
        ]
        : [
          '--bootnodes',
          `/ip4/127.0.0.1/tcp/${ports['alice'].p2p}/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp`,
        ]),
      // only print logs from the alice node
      ...(authority === 'alice'
        ? [
          '-lruntime::offchain=debug',
          '--rpc-cors',
          'all',
          '--rpc-methods=unsafe',
          '--unsafe-rpc-external',
        ]
        : []),
    ],
    {
      cwd: gitRoot,
    }
  );

  __NODE_STATE[authority].isRunning = true;
  __NODE_STATE[authority].process = proc;

  if (options.printLogs) {
    proc.stdout.on('data', (data) => {
      console.log(`${authority}: ${data}`);
    });
    proc.stderr.on('data', (data) => {
      console.error(`${authority}: ${data}`);
    });
  }

  proc.on('close', (code) => {
    __NODE_STATE[authority].isRunning = false;
    __NODE_STATE[authority].process = null;
    console.log(`${authority} node exited with code ${code}`);
  });
  return proc;
}

export function purgeNode(
): child.ChildProcess {
  const gitRoot = child
  .execSync('git rev-parse --show-toplevel')
  .toString()
  .trim();
  const nodePath = `${gitRoot}/target/release/firechain-node`;

  const command = `
    yes | ${nodePath} \
    --purge-chain \
    --chain qa-dev
  `;

  const proc = child.spawn(
    'sh',
    ['-c', command], // Execute the command using "sh"
    {
      cwd: gitRoot,
    }
  );


  proc.stdout.on('purge data', (data) => {
    console.log(`: ${data}`);
  });
  proc.stderr.on('purge error data', (data) => {
    console.error(`: ${data}`);
  });


  proc.on('close', (code) => {
    console.log(` finished purging exited with code ${code}`);
  });
  return proc;
}

/**
 * Waits until a new session is started.
 */
export async function waitForTheNextSession(api: ApiPromise): Promise<void> {
  return waitForEvent(api, 'session', 'NewSession');
}

/**
 * Wait and listen for an event
 */
export async function waitForEvent(
  api: ApiPromise,
  pallet: string,
  eventVariant: string,
  dataQuery?: { key: string }
): Promise<void> {
  return new Promise(async (resolve, _rej) => {
    while (true) {
      // Subscribe to system events via storage
      const events = await api.query.system.events();
      const eventsJson = events.toJSON();
      const eventsValue = api.registry.createType("Vec<EventRecord>", events.toU8a());
      // Loop through the Vec<EventRecord>
      for (var event of eventsValue) {
        //console.log("Checking event: ", event);
        // @ts-ignore
        const section = event.event.section;
        // @ts-ignore
        const method = event.event.method;
        // @ts-ignore
        const data = event.event.data;
        //console.log("Event section = ", section, ", method = ", method);
        //console.log("Event musteq  = ", pallet, ", method = ", eventVariant);
        if (section === pallet && method === eventVariant) {
          // console.log(
          //   `Event ($section}.${method}) =>`,
          //   data
          // );
          if (dataQuery) {
            for (const value of data) {
              const jsonData = value.toJSON();
              if (jsonData instanceof Object) {
                Object.keys(jsonData).map((key) => {
                  if (key === dataQuery.key) {
                    return resolve(void 0);
                  }
                });
              }
            }
          } else {
            return resolve(void 0);
          }
        }
      }

      await sleep(2000);
    }
  });
}

export async function sudoTx(
  api: ApiPromise,
  call: SubmittableExtrinsic<'promise'>
): Promise<void> {
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  const unsub = await api.tx.sudo
      .sudo(call.method.toHex())
      .signAndSend(alice, {tip: 2000, nonce: -1}, (result ) => {
        if (result.status.isInBlock) {
          console.log(`Sudo transaction included at blockHash ${result.status.asInBlock}`);
          console.log(`Waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          // @ts-ignore
          const data = JSON.stringify(result.events);
          console.log(data);
          unsub();
        }
      });
}

export async function uncheckedSudoTx(
  weight: DetectCodec<any, any>,
  api: ApiPromise,
  call: SubmittableExtrinsic<'promise'>
): Promise<void> {
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');

  return new Promise(async (resolve, _reject) => {
    const unsub = await api.tx.sudo
      .sudoUncheckedWeight(call, weight)
      .signAndSend(alice, {tip: 2000, nonce: -1}, ({ status }) => {
        if (status.isFinalized) {
          unsub();
          resolve();
        }

      });
  });
}


export function ethAddressFromUncompressedPublicKey(
  publicKey: `0x${string}`
): `0x${string}` {
  const pubKeyHash = ethers.utils.keccak256(publicKey); // we hash it.
  const address = ethers.utils.getAddress(`0x${pubKeyHash.slice(-40)}`); // take the last 20 bytes and convert it to an address.
  return address as `0x${string}`;
}
