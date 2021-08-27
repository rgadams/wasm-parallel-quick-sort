import { join } from 'lodash';
import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';

let wasmPkg;

const N = 1000000;

function component() {
    const element = document.createElement('div');

    // Lodash, currently included via a script, is required for this line to work
    element.innerHTML = join(['Hello', 'webpack'], ' ');

    return element;
}

async function load_wasm() {
    const module = await import('../../quick-sort/pkg/quick_sort');
    await module.default();
    await module.initThreadPool(navigator.hardwareConcurrency);
    return module;
}

// http://stackoverflow.com/questions/962802#962890
function shuffle(array: number[]) {
    var tmp, current, top = array.length;
    if (top) while (--top) {
        current = Math.floor(Math.random() * (top + 1));
        tmp = array[current];
        array[current] = array[top];
        array[top] = tmp;
    }
    return array;
}

function getUnsortedArray() {
    for (var a = [], i = 0; i < N; ++i) a[i] = i;

    return shuffle(a);
}

load_wasm().then((module) => {
    const unsorted_array = getUnsortedArray();
    const sorted_array = module.quick_sort_timed(new Int32Array(unsorted_array));
    const parallel_sorted_array = module.quick_sort_parallel_timed(new Int32Array(unsorted_array));
    const sorted_arry_v2 = module.quick_sort_v2_timed(new Int32Array(unsorted_array));
    const parallel_sorted_array_v2 = module.parallel_quick_sort_v2_timed(new Int32Array(unsorted_array));
});

