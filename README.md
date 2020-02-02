# Clatter Rattles RS

![Build](https://github.com/bartOssh/clatter_rattles_rs/workflows/Rust/badge.svg?branch=master)

<p align="center">
    <img 
    width="85%" height="85%"
    src="https://github.com/bartOssh/clatter_rattles_rs/blob/master/assets/logo.jpg"/>
</p>

## Description


:headphones: Sound pattern recognition library.

- Offers fingerprint hashing mechanism for the sample fingerprints and the stream chunk.
- Offers recognition from local file.
- Offers recognition from sound stream (Now only mp3 radio stream) based
    on [Fast Fourier Transform](https://en.wikipedia.org/wiki/Fast_Fourier_transform)
    and [Fingerprint Hash](https://en.wikipedia.org/wiki/Fingerprint_(computing)).
- Main focus of this library is to offer parallel way of stream chunk fingerprint hashing and
    then comparing them with stored fingerprints. So the app running library can offer constant
    watch over sound stream for a look of corresponding best matching sample.
- Secondary feature is to offer easy callable function for the best matching reflection of the
    given sample in database collection. This approach is focusing on the fastest way by leveraging
    database hash lookup. To achieve that only small bit of stream or file chunk is hashed.
- Library uses build in solution to handle fingerprints storage. This library offers [Redis](https://redis.io/)
    database support for storing hashes and finding matches. But by using public `trait Repository`
    it is fairly easy to implement this feature for any database. It is highly recommended
    to use database engin that supports fast hash lookup and not recommended to use relational database.
    Best performance will be achieved on fairly simple schema in RAM database such as Redis.
- Last but not least this library has async stream request and parallel stream hashing build in,
    so there is no need for building parallel mechanism by yourself. Stream listener collects asynchronously
    stream chunks and decodes them in separate thread than main library, then pipes it to receiver.
    Receiver is boxed by atomic type so hashing and matching can be done in main thread or can be send to
    thread worker. This design allows for use as much processor resources as possible and
    speed up matching algorithm significantly.
    The weakest point so far is that only one thread can read and write to database, and main cause for that is
    collecting finding all wasn't showing any significant improvement of performance while iterating in parallel.
    Even calling for hash matching in larger database was showing slower performance when done in parallel.
    So finally single threaded database hash matching has been chosen.

### Testing

- Run local redis database with `systemctl start redis-server.service`
- Test it with `cargo test -- --nocapture` or `cargo test` <- with less information

### Building

- Building. Please build with `RUSTFLAGS="--emit=asm"` flag enabled, force the compiler to use a single LLVM module for the entire crate which allows LLVM to optimize better.

## License

[MIT](https://github.com/bartOssh/clatter_rattles_rs/blob/master/LICENSE)

## Dedication

:cherry_blossom: Dla Grzechotki :cherries:
