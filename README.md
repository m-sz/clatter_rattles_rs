# Clatter Rattles RS

![Build](https://github.com/bartOssh/clatter_rattles_rs/workflows/Rust/badge.svg?branch=master)

<p align="center">
    <img 
    width="85%" height="85%"
    src="https://github.com/bartOssh/clatter_rattles_rs/blob/master/assets/logo.jpg"/>
</p>

## Description

- Sound pattern recognition library.
- Offers fingerprint hashing mechanism for the sample fingerprints and the stream chunk.
- Offers recognition from local file.
- Offers recognition from sound stream (Now only mp3 radio stream) based on [Fast Fourier Transform](https://en.wikipedia.org/wiki/Fast_Fourier_transform) and [Fingerprint Hash](https://en.wikipedia.org/wiki/Fingerprint_(computing)).
- Main focus of this library is to offer parallel way of comparing stream chunk fingerprint hash with sample fingerprint hashes stored in collection. So the app running library can offer constant watch over sound stream for a look of corresponding best fitted sample. Fingerprints of a stream chunk are constantly compared by parallel workers with stored in collections samples of fingerprint hashes. This feature will work smoothly only for small collection of samples for one stream.
- Secondary feature is to offer finding samples of fingerprint hashes in the collection that are the most fitted reflection of the given sample to compare against. This approach is focusing on the fastest way of leveraging database hash lookup. To achieve that only small bit of stream or file chunk is hashed.
- Library uses build in solution to handle fingerprints storage. This library offers [Redis](https://redis.io/) database support for storing hashes and finding matches. But by using public `trait Repository` it is fairly easy to implement this feature for any database. It is highly recommended to use database engin that supports fast hash lookup. Don't recommended to use relational database.

### Testing

- Run local redis database with `systemctl start redis-server.service`
- Test it with `cargo test -- --nocapture` or `cargo test` <- with less information

### Building

- Building. Please build with `RUSTFLAGS="--emit=asm"` flag enabled, force the compiler to use a single LLVM module for the entire crate which allows LLVM to optimize better.


## License

[MIT](https://github.com/bartOssh/clatter_rattles_rs/blob/master/LICENSE)

## Dedication

:cherry_blossom: Dla Grzechotki :cherries:
