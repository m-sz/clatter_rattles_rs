# Clatter Rattles RS

![Build](https://github.com/bartOssh/clatter_rattles_rs/workflows/Rust/badge.svg?branch=master)

<p align="center">
    <img 
    width="85%" height="85%"
    src="https://github.com/bartOssh/clatter_rattles_rs/blob/master/assets/logo.jpg"/>
</p>

## Description


:headphones: Sound pattern recognition library.

Features:

- fingerprint hashing mechanism based on
    [Fast Fourier Transform](https://en.wikipedia.org/wiki/Fast_Fourier_transform)
    and [Fingerprint Hash](https://en.wikipedia.org/wiki/Fingerprint_(computing)).
- matching sample from local file.
- matching sample from sound stream (Now only mp3 radio stream).
- focuses to offer parallel way of stream chunk fingerprint hashing.
- tries to offer simple way to match given sample against songs in database collection.
- offers build in solution to handle fingerprints storage. This library offers [Redis](https://redis.io/)
    database support for storing hashes and finding matches. By using `trait Repository`
    it is fairly easy to implement this feature for any database. It is highly recommended
    to use database engine that supports fast hash lookup.
    Best performance will be achieved on fairly simple schema with in RAM database.
- provides async stream request and parallel stream hashing.
    There is no need for building parallel mechanism by yourself.
    Stream listener collects asynchronously stream chunks and decodes them in separate thread than main library,
    then pipes it to receiver. Receiver is boxed by atomic type so hashing and matching can be done in main thread
    or can be send to thread worker.
    This design allows for use as much processor resources as possible and speed up matching algorithm significantly.
- the weakest point so far is that only one thread can read and write to database. 
    Main cause for that is that using hash lookup wasn't showing any significant improvement of performance while
    performed in parallel threads.
    More then that, calling for hash matching in larger database was showing slower performance when done in single thread.

### Testing

- Run local redis database with `systemctl start redis-server.service`
- Test it with `cargo test -- --nocapture` or `cargo test` <- with less information

### Building

- Building. Please build with `RUSTFLAGS="--emit=asm"` flag enabled, force the compiler to use a single LLVM module for the entire crate which allows LLVM to optimize better.

## License

[MIT](https://github.com/bartOssh/clatter_rattles_rs/blob/master/LICENSE)

## Dedication

:cherry_blossom: Dla Grzechotki :cherries:
