

## Description

The goal of this project is to convert Vitalik's Python implementation of BN128 pairing into Rust. While the conversion is not fully complete, key components such as the field elements and extension fields for performing the pairing have been implemented. 

- **G1**: A field element of `FQ2`.
- **G2**: A field element of the extension of `FQ2`, which is `FQ12`.

Pairing is a mathematical operation that involves two elliptic curve points to produce a scalar output. In this repository, we focus on implementing **Miller pairing**, a well-known pairing algorithm.


## Pairing Algorithms

There are several types of pairing algorithms, and this repository implements **Miller pairing**. Future updates may include additional algorithms and optimizations.

## Resources for Extension Fields and Pairing

- [Exploring Elliptic Curve Pairings](https://medium.com/@VitalikButerin/exploring-elliptic-curve-pairings-c73c1864e627)
https://0xparc.org/blog/zk-pairing-2
- [How We Implemented the BN254 Ate Pairing in Lambdaworks](https://blog.lambdaclass.com/how-we-implemented-the-bn254-ate-pairing-in-lambdaworks/)


