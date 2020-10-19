# rust-opse
Order-Preserving Symetrical Encryption algorithm for rust.

This work is an implementation of Boldyreva's symmetric order-preserving encryption scheme ([Boldyreva's paper](http://www.cc.gatech.edu/~aboldyre/papers/bclo.pdf)).
The implementation is based on tonyo's pyope package implementation, also based on Boldyreva's paper.

**Disclaimer 1** This is an experimental implementation, which should be thoroughly reviewed and evaluated before using in production and/or sensitive applications.

**Disclaimer 2** The Boldyreva scheme is not a standardized algorithm, so there are no test vectors and fixed plaintext-ciphertext mapping for a given key. It means that, generally speaking, a plaintext encrypted with the same key by two different versions of the package might not be equal to each other.

Running tests
-------------
Cargo can be used for all tests. Symply run the following in a shell with a valid rustc installed

```shell
$ cargo test
```
