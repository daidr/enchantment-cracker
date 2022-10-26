import init, { first_input } from "@rsw/rng-worker";

let THREAD_COUNT, THREAD_INDEX;

async function initEnv({ threadCount, threadIndex } = {}) {
    THREAD_COUNT = threadCount;
    THREAD_INDEX = threadIndex;
    const module = await init();
    module.memory.grow(10000);
}

self.addEventListener('message', (e) => {
    switch (e.data.type) {
        case 'firstInput':
            firstInput(e.data.payload);
            break;
        case 'initEnv':
            initEnv(e.data.payload);
            break;
    }
})

function firstInput({ bookshelves, slot1, slot2, slot3, seedSharedBuf }) {
    let time = performance.now();
    let ret = first_input(Number(bookshelves), Number(slot1), Number(slot2), Number(slot3), Number(THREAD_COUNT), seedSharedBuf)
    console.log(`firstInput took ${performance.now() - time}ms`);

    self.postMessage({
        type: 'firstInputDone',
        payload: {
            count: ret,
            inputData: {
                bookshelves,
                slot1,
                slot2,
                slot3,
                seedSharedBuf
            }
        }
    });
}

