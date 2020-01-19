# Clatter Rattles RS

<p align="center">
    <img 
    width="100%" height="100%"
    src="https://github.com/bartOssh/clatter_rattles_rs/blob/master/assets/logo.jpg"/>
</p>

- Library for sound pattern recognition from sound stream based on [Fast Fourier Transform](https://en.wikipedia.org/wiki/Fast_Fourier_transform) and [Fingerprint Hash](https://en.wikipedia.org/wiki/Fingerprint_(computing)).
- Offers fingerprint hashing mechanism for the sample fingerprints and the stream chunk.

- Main focus of this library is to offer async way of comparing stream chunk fingerprint hash with sample fingerprint hashes stored in collection. So the app running library can offer constant watch over sound stream for a look of corresponding - the best fitting - sample in the collection. Fingerprints of a stream chunk are constantly asynchronously compared with stored in collections samples of fingerprint hashes. This feature will work smoothly only for small collection of samples.

- Secondary feature is to find samples of fingerprint hashes in the collection that are the most fitted reflection of the given sample to compare against. This approach is focusing on the fastest way of comparing hashes by database. 

## Description

## WORK IN PROGRESS


### Until first release and only one contributor all the commits are going to be pushed to the master branch. For more contributors and after release other I will decide on other, more suitable approach
