/* __BEGIN_OF: PRELUDE */
/* tslint:disable */
/* eslint-disable */
(function (global, TextDecoder, factory) {
    if (typeof define === "function" && define.amd) {
        define("QMC2CryptoModule", [], () => factory(TextDecoder));
    } else if (typeof module === "object" && typeof module.exports === "object") {
        module.exports = factory(TextDecoder);
    } else {
        global.QMC2CryptoModule = factory(TextDecoder);
    }
})(
    typeof window !== "undefined" ? window : this,
    typeof TextDecoder === 'undefined' ? (0, require('util')).TextDecoder : TextDecoder,
    function (TextDecoder) {
        const cachedTextDecoder = new TextDecoder('utf-8', {ignoreBOM: true, fatal: true});
        cachedTextDecoder.decode();

        const __wasm_blob = Uint8Array.from(atob("${WASM_B64}"), c => c.charCodeAt());
        let __last_inst;

        const exports = (function () {
            /* __END_OF: PRELUDE */

            /* __BEGIN_OF: INJECT_WRAPPER */
            function injectToExports(instance) {
                const exports = Object.create(null);
                exports._instance = instance;
                // __INJECTION__
                instance.__init();
                return exports;
            }
            /* __END_OF: INJECT_WRAPPER */

            /* __BEGIN_OF: CLOSE_EXPORTS */
            return async function () {
                const instance = await init.apply(this, arguments);
                // Don't overwrite.
                if (!__last_inst) {
                    __last_inst = instance;
                }
                return injectToExports(instance);
            };
        })();
        /* __END_OF: CLOSE_EXPORTS */

        /* __BEGIN_OF: ENDING */
        return exports;
    });
/* __END_OF: ENDING */
