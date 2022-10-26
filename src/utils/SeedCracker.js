import { useMessage } from "@/components/Message";
import RNGWorker from "@/worker/rng-worker?worker";
import init, { get_last_few_seeds as getLastFewSeeds } from "@rsw/rng-worker";
import { useI18n } from "vue-i18n";

const { info } = useMessage();
const { t } = useI18n();

export class SeedCracker {
    // debug state
    _DEBUG = true;
    // thread number: the count of threads, can be set via constructor
    _ThreadCount = 1;
    // the instance of workers
    _WorkerPoolList = [];

    _IsFirstTime = true;

    /** SHARED_BUFFER BEGIN */

    // which could terminate running workers if the first byte set to 1
    // Int8Array(1 byte) and default value is 0
    // AbortRequestSharedBuffer = new SharedArrayBuffer(1);

    /** SHARED_BUFFER END */

    _LastFewSeedsList = [];

    /** firstInput Global states BEGIN */

    _FirstInputTimerId = -1;
    _FirstInputIsProsessing = false;
    _FirstInputDoneThreadsCount = 0;
    _FirstInputDoneSeedsCount = 0;

    /** firstInput Global states END */

    constructor({ threadCount = 2 }) {
        this._ThreadCount = threadCount;
        init();
    }

    isFirstTime() {
        return this._IsFirstTime;
    }

    // call this function to reset the cracker
    // including reset the shared buffer and terminate all workers
    // NOTE: we should call this function while user clicked the reset button
    resetCracker() {
        // remove old workers first
        while (this._WorkerPoolList.length > 0) {
            this._WorkerPoolList.pop().terminate()
        }

        // set AbortRequestSharedBuffer to 0
        // new Int8Array(this.AbortRequestSharedBuffer)[0] = 0;

        this._IsFirstTime = true;
        this._FirstInputDoneThreadsCount = 0;
        this._FirstInputDoneSeedsCount = 0;
        this._FirstInputTimerId = -1;
        this._FirstInputIsProsessing = false;

        info(t("rngCracker.message.abort"));

        return this.initCracker();
    }

    // call this function to init the cracker
    initCracker() {
        try {
            // create new workers
            for (let i = 0; i < this._ThreadCount; i++) {
                const worker = new RNGWorker();

                // listen for messages from the worker
                worker.addEventListener("message", (e) => {
                    this._workerHandler(e.data);
                });

                // call initEnv function in workers
                worker.postMessage({
                    type: "initEnv",
                    payload: { threadCount, threadIndex: i },
                });

                // add to worker pool
                workerPool.push(worker);
            }
        } catch (error) {
            if (this._DEBUG) console.log(error)
            return false;
        }
        return true;
    }

    firstInput(bookshelves, slot1, slot2, slot3, progressHandler) {
        // store the seed cursor while will be used in all workers
        //  Int32Array(4 bytes) and default value is -2147483648
        // NOTE: if we want to write a value to the shared buffer, we should use Atomics.store
        const seedSharedBuf = new SharedArrayBuffer(4);

        if (this._DEBUG) console.log("firstInput: ", bookshelves, slot1, slot2, slot3);

        // init seedSharedBuf
        Atomics.store(new Int32Array(seedSharedBuf), 0, -2147483648);

        // remove old timer
        clearInterval(this._FirstInputTimerId);

        this._FirstInputIsProsessing = true;

        const seedSharedBufView = new Int32Array(seedSharedBuf);
        setInterval(() => {
            if (this._FirstInputIsProsessing) {
                // get the percentage of progress
                const progress = (seedSharedBufView[0] + 2147483648) / 4294967296 * 100;
                // call the progress handler
                progressHandler(progress, true);
            } else {
                progressHandler(0, false);
                clearInterval(this._FirstInputTimerId);
            }
        }, 200)

        // dispatch the firstInput task to all workers
        this._WorkerPoolList.forEach((worker) => {
            worker.postMessage({
                type: "firstInput",
                payload: {
                    bookshelves,
                    slot1,
                    slot2,
                    slot3,
                    seedSharedBuf,
                },
            });
        });
    }

    _workerHandler({ type, payload }) {
        switch (type) {
            case "firstInputDone":
                this._firstInputDoneHandler(payload);
                break;
        }
    }

    _firstInputDoneHandler({
        count,
        inputData: { bookshelves, slot1, slot2, slot3, seedSharedBuf }
    }) {
        this._FirstInputDoneThreadsCount++;
        this._FirstInputDoneSeedsCount += count;
        if (_FirstInputDoneThreadsCount === this._ThreadCount) {
            this._FirstInputIsProsessing = false;

            if (this._DEBUG) console.log("firstInputDone(before): ", this._FirstInputDoneSeedsCount);

            {
                // find seed not be validated
                const seedSharedDataView = new Int32Array(seedSharedBuf);
                const seed = Atomics.load(seedSharedDataView, 0);
                this._LastFewSeedsList = getLastFewSeeds(
                    seed,
                    bookshelves,
                    slot1,
                    slot2,
                    slot3
                );

                const totalCount = this._FirstInputDoneSeedsCount + this._LastFewSeedsList.length;

                if (this._DEBUG) console.log("firstInputDone(after): ", totalCount);

                info(t("rngCracker.message.finished"));
            }

            // reset the global states
            this._FirstInputDoneSeedsCount = 0;
            this._FirstInputDoneThreadsCount = 0;
            this._IsFirstTime = false;
        }
    };
}