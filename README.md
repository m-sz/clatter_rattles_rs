# Clatter Rattles RS

![v0.1.0](https://github.com/bartOssh/clatter_rattles_rs/workflows/Rust/badge.svg?branch=master)

<p align="center">
    <img 
    width="70%" height="70%"
    src="https://github.com/bartOssh/clatter_rattles_rs/blob/master/assets/logo.jpg"/>
</p>

## Description

- Library for sound pattern recognition from sound stream based on [Fast Fourier Transform](https://en.wikipedia.org/wiki/Fast_Fourier_transform) and [Fingerprint Hash](https://en.wikipedia.org/wiki/Fingerprint_(computing)).
- Offers fingerprint hashing mechanism for the sample fingerprints and the stream chunk.

- Main focus of this library is to offer parallel way of comparing stream chunk fingerprint hash with sample fingerprint hashes stored in collection. So the app running library can offer constant watch over sound stream for a look of corresponding best fitted sample. Fingerprints of a stream chunk are constantly compared by parallel workers with stored in collections samples of fingerprint hashes. This feature will work smoothly only for small collection of samples for one stream.

- Secondary feature is to offer finding samples of fingerprint hashes in the collection that are the most fitted reflection of the given sample to compare against. This approach is focusing on the fastest way of leveraging database hash lookup.

## Development

### Testing

- Run local redis database with `systemctl start redis-server.service`
- Test it with `cargo test -- --nocapture`

### Building

- Building. Please build with `RUSTFLAGS="--emit=asm"` flag enabled, force the compiler to use a single LLVM module for the entire crate which allows LLVM to optimize better.

## WORK IN PROGRESS - TODO

- stream tracker for analysis of in memory fingerprint collection reference to trigger response when match happen
- flexible matching for in memory fingerprint collection tracking

### FIRST VERSION NOT READY YET

## Dedication

:cherry_blossom: Dla Grzechotki :cherries:
