# Implementations from Fully Homomorphic Encryption (FHE) and Lattice-based Cryptography

## Public Key Cryptography based on Learning With Errors (LWE)

[Check out implementation!](/src/pub_lwe.rs)

### SETUP
1. Pick $q$, $m$, $n$ and sigma where $q$ is the modulus, $m$ and $n$ are positive integers  and sigma is the standard deviation for the discrete guassian distribution

### PROCESS
1. Generate the public key $(A, b)$ 
    1. Sample random matrix: $A \leftarrow Z_q^{m \times n}$
    2. Sample random vector: $s \leftarrow Z_q^n$.  $s$ is your secret key 
    3. Sample random error vector: $e \leftarrow X^m$ (Small noise e.g discrete guassian)
    4. Compute $b = As + e \pmod{q}$ 
2. Encryption:
    1. Sample a random bit vector: $r \leftarrow \{ 0, 1 \}^m$
    2. Pick your message $m$ ($m$ is $0$ or $1$) and compute $(u, v)$
    3. $u = A^{\top}r \pmod{q}$ where $A^{\top}$ is transpose of $A$
    4. $v = b^{\top}r + m \cdot \lfloor q/2 \rfloor \pmod{q}$ where $b^{\top}$ is the transpose of $b$ and $\lfloor q/2 \rfloor$ is the floor of $q/2$
3. Decryption:
    1. Compute $v - \langle u, s \rangle$ where $\langle u, s \rangle$ is the inner product of vectors $u$ and $s$. The result should be $e^{\top}r + m \cdot \lfloor q/2 \rfloor$
    2. Decoding: if result is closer to $0$, decrypt to $0$ and if close to $q/2$ decrypt to $1$
 
### CORRECTNESS CHECK
For decryption to work properly, you have to pick $q$, $m$ and sigma so that $|e^Tr| \leq q/4$. 

## Residue Number System

[Check out implementation!](/src/rns.rs)

