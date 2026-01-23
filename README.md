# Implementations from Fully Homomorphic Encryption (FHE) and Lattice-based Cryptography

## Public Key Cryptography based on Learning With Errors (LWE)

[Check out the implementation!](/src/pub_lwe.rs)

### Setup
1. Pick $q$, $m$, $n$ and sigma where $q$ is the modulus, $m$ and $n$ are positive integers  and sigma is the standard deviation for the discrete guassian distribution

### Process
1. **Generate the public key $(A, b)$**: 
    1. Sample random matrix: $A \leftarrow Z_q^{m \times n}$
    2. Sample random vector: $s \leftarrow Z_q^n$.  $s$ is your secret key 
    3. Sample random error vector: $e \leftarrow X^m$ (Small noise e.g discrete guassian)
    4. Compute $b = As + e \pmod{q}$ 
2. **Encryption**:
    1. Sample a random bit vector: $r \leftarrow \lbrace 0, 1 \rbrace^m$
    2. Pick your message $m$ ($m$ is $0$ or $1$) and compute $(u, v)$
    3. $u = A^{\top}r \pmod{q}$ where $A^{\top}$ is transpose of $A$
    4. $v = b^{\top}r + m \cdot \lfloor \dfrac{q}{2} \rfloor \pmod{q}$ where $b^{\top}$ is the transpose of $b$ and $\lfloor \dfrac{q}{2} \rfloor$ is the floor of $\dfrac{q}{2}$
3. **Decryption**:
    1. Compute $v - \langle u, s \rangle$ where $\langle u, s \rangle$ is the inner product of vectors $u$ and $s$. The result should be $e^{\top}r + m \cdot \lfloor \dfrac{q}{2} \rfloor$
    2. Decoding: if result is closer to $0$, decrypt to $0$ and if close to $\dfrac{q}{2}$ decrypt to $1$
 
### Correctness Check
For decryption to work properly, you have to pick $q$, $m$ and sigma so that $|e^Tr| \leq \dfrac{q}{4}$. 

## Residue Number System (RNS)

[Check out the implementation!](/src/rns.rs)

RNS is a mechanism used in multiple FHE schemees to operate on large numbers exceeding the limits available in the cryptosystem.

This is simply a way of breaking a number into a tuple of numbers and this is made possible by the **Chinese Remainder Theorem**.

That is, given a number $x$, we can represent it as $(x_1, x_2, \ldots, x_k)$ where each $x \equiv x_i \pmod{n_i}$ for some choices of $n_i$ (they must be co-prime, and their product must be larger than $x$).

To get $x$ from the tuple, we use the formula: $$x = \sum_{i =0}^{k}x_iM_iQ_i\pmod{Q}$$ 
where:
- $k$ is the number of elements in the tuple
- $Q = n_1\cdot n_2 \cdot \cdots n_k$
- $Q_i = Q/n_i$
- $M_i = Q_i^{-1} \pmod{n_i}$. That is, multiplicative inverse of $Q_i$ mod $n_i$

Let's see an example.

Given $x = 52$ and $k = 3$, we pick $n_1 = 3$, $n_2 = 5$ and $n_3 = 7$. We have the tuple $(1, 2, 3)$ as: $$52 \equiv 1 \pmod{3}$$ $$52 \equiv 2 \pmod{5}$$ $$52 \equiv 3 \pmod{7}$$

Now, let's walk backwards to get $x$.

- $Q = n_1 \cdot n_2 \cdot n_3 = 3 \cdot 5 \cdot 7 = 105$
- $Q_i$
    - $Q_1 = Q/n_1 = 105/3 = 35$
    - $Q_2 = Q/n_2 = 105/5 = 21$
    - $Q_3 = Q/n_3 = 105/7 = 15$
- $M_i$
    - $M_1 = Q_1^{-1}\pmod{n_1} = 35^{-1} \pmod{3} = 2$
    - $M_2 = Q_2^{-1}\pmod{n_2} = 21^{-1} \pmod{5} = 1$
    - $M_3 = Q_3^{-1}\pmod{n_3} = 15^{-1} \pmod{7} = 1$
- $x = (x_1 \cdot M_1 \cdot Q_1) + (x_2 \cdot M_2 \cdot Q_2) + (x_3 \cdot M_3 \cdot Q_3) \pmod{Q} = (1 \cdot 2 \cdot 35) + (2 \cdot 1 \cdot 21) + (3 \cdot 1 \cdot 15) \pmod{105} = 70 + 42 + 45 \pmod{105} = 52 \pmod{105}$

The main benefit of using the tuple is that we can perform operations on them and go back to $x$ afterwards.