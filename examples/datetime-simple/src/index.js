import * as wasm from 'datetime-simple';

(async () => {
    await wasm;
    const result = wasm.add_next_day(
        '2020-01-01'
    ); // WebAssemblyのエクスポートされた関数を呼び出す
    console.log('Result:', result);
})();
