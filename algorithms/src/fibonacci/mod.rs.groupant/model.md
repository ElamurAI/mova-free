# algorithm-fibonacci

## descriptant

### algorithm-fibonacci

Predicted cost: O(log(n+1)) fixed-word arithmetic, at most64 rounds, O(1) storage. Independent iterative and 2x2 matrix references have finite fixture domains.

Compiled controls are rejected and exact bytes restored. Six file roots contain36 typed views, no fragments. Twice-identical partial syntax binds source; macros remain opaque. No semantic AST, formal proof, approximate surrogate or speed claim.

## model

# algorithm-fibonacci

## Forward

For all u64 n and nonzero m return F(n) mod m; F(0)=0,F(1)=1. m=0 returns ZeroModulus before arithmetic. Scan index bits preserving consecutive residues; use c=a(2b-a),d=a^2+b^2 mod m. Reference: Knuth, The Art of Computer Programming, volume1, section1.2.8.

## Invariant

Residues below m; each product fits u128. Reduce each square before adding because raw sum may exceed u128. 2b+m-a is nonnegative and below3m. No index increment, including n=u64::MAX.

## Cost

O(log(n+1)) fixed-word arithmetic, at most64 rounds, O(1) storage. Independent iterative and 2x2 matrix references have finite fixture domains.

## behavior_complement

Universe (u64 n,u64 m) x Result<u64,error>. m=0 excludes every output except ZeroModulus; m>0 excludes errors and values unequal to recurrence residue. m=1 excludes every result except Ok(0). This target does not turn finite evidence into proof.

## reverse_reconstruction

For fixed m,r retain all n with F(n)=r mod m. Fibers generally contain many indices; m=1 retains every u64 index. Without fixed modulus retain the union over nonzero moduli. ZeroModulus forgets n entirely.

## doubling_state

After prefix k, (a,b)=(F(k),F(k+1)) mod m. Swapping even assignment compiles and fails exhaustive_iterative_oracle.

## wide_arithmetic

Squares individually reduce before sum. Near-max modulus fixtures exercise full-width multiplication, and matrix reference reduces each dot-product term.

## matrix_oracle

Generic binary exponentiation of [[1,1],[1,0]] independently checks full-width indices including u64::MAX.

## recurrence_boundary

Iterative reference checks n<=1024 for m1..128 and two maximal moduli. Large recurrence uses n<=u64::MAX-2; zero/one moduli tested at maximal index.

## evidence

### Evidence

Nine tests total, three per algorithm, pass twice debug and twice release with identical normalized outcomes. Three compiled production controls fail exhaustive_gcd_oracle, exhaustive_factor_oracle and exhaustive_iterative_oracle; exact bytes restored before positives. Strict all-target Clippy and narrow formatting pass. External wave16 numeric bundle retains logs. Finite checks do not establish universal correctness or allocator success.

## History

### 2026-10-30 · descriptant

Epoch2026-10-30: original implementation and independent fixtures. Prospective relation: For all u64 n and nonzero m return F(n) mod m; F(0)=0,F(1)=1. m=0 returns ZeroModulus before arithmetic. Scan index bits preserving consecutive residues; use c=a(2b-a),d=a^2+b^2 mod m. Reference: Knuth, The Art of Computer Programming, volume1, section1.2.8.

### Undated · evidence

Source SHA-256: 508f3ef6ce9b93a1913ece15e64425cd0e200ea24adda0059c2daa831f271925.

- debug-1-fibonacci.log: b25fa6effa9a946a2734d7e40a3a501538ce58d830b08a93d9d9063a7c734168

- debug-2-fibonacci.log: bca61daa9d2d6df6276c03bf11cd283ae6aa76bfab8ee02aad017e8ae0a5c994

- release-1-fibonacci.log: 6f8db96d2710a8b290bf3ceab53c8a40e2cc83829cf0d88e7bd9e51008fea725

- release-2-fibonacci.log: 6f8db96d2710a8b290bf3ceab53c8a40e2cc83829cf0d88e7bd9e51008fea725

- mutant-fibonacci.log: f3e531d2d06cf700d87de74eebd1df5056ee19fee8a239271d752dbdb5378b7d
