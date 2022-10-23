import init, { first_input } from "@rsw/rng-worker";

let CORE_COUNT, CORE_INDEX;

async function initEnv({ coreCount, coreIndex } = {}) {
    CORE_COUNT = coreCount;
    CORE_INDEX = coreIndex;
    const module = await init();
    module.memory.grow(10000);
    // const myRNG = SimpleRandom.new();
    // myRNG.set_seed(2521490391n);
    // console.log(myRNG.next_int(8));
    // console.log(myRNG.next_int(8));
    // console.log(myRNG.next_int(8));
    // console.log(myRNG.next_int(8));
    // console.log(myRNG.next_int(8));
    // console.log(myRNG.next_int(8));
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

function firstInput({ bookshelves, slot1, slot2, slot3, seedSharedBuf, seedSearchedSharedBuf, abortRequestedSharedBuf }) {
    let time = performance.now();
    let a = first_input(Number(bookshelves), Number(slot1), Number(slot2), Number(slot3), Number(CORE_COUNT), seedSharedBuf, seedSearchedSharedBuf, abortRequestedSharedBuf)
    console.log(`firstInput took ${performance.now() - time}ms`);
    self.postMessage({ type: 'firstInputDone', payload: a });
}

