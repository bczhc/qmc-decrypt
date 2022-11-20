# qmc2-crypto-rust

Rusty implementation of QMC2-Crypto, using WebAssembly

## Usage

```js
const QMCCrypto = require("@jixun/qmc2-crypto-rust");
QMCCrypto().then(QMCCrypto => {
    // ...
});

// After initialisation, methods can be called directly.
QMCCrypto.detect(data);
```
