# algorithm-fibonacci-tests

## descriptant

### algorithm-fibonacci-tests

Predicted cost: O(log(n+1)) fixed-word arithmetic, at most64 rounds, O(1) storage. Independent iterative and 2x2 matrix references have finite fixture domains.

Compiled controls are rejected and exact bytes restored. Six file roots contain36 typed views, no fragments. Twice-identical partial syntax binds source; macros remain opaque. No semantic AST, formal proof, approximate surrogate or speed claim.

## model

# algorithm-fibonacci-tests

## Forward

Finite executable observation: For all u64 n and nonzero m return F(n) mod m; F(0)=0,F(1)=1. m=0 returns ZeroModulus before arithmetic. Scan index bits preserving consecutive residues; use c=a(2b-a),d=a^2+b^2 mod m. Reference: Knuth, The Art of Computer Programming, volume1, section1.2.8. Assertions check independent arithmetic oracles, boundaries and identities; passing fixtures do not establish untested behavior.

## Invariant

Residues below m; each product fits u128. Reduce each square before adding because raw sum may exceed u128. 2b+m-a is nonnegative and below3m. No index increment, including n=u64::MAX.

## Cost

O(log(n+1)) fixed-word arithmetic, at most64 rounds, O(1) storage. Independent iterative and 2x2 matrix references have finite fixture domains.

## behavior_complement

Universe is fixture executions and assertion outcomes. A passing actual/expected mismatch is excluded; failure cannot count as success. This finite complement does not cover all API calls.

## reverse_reconstruction

Passing summary retains every implementation agreeing on exercised fixtures, including implementations wrong elsewhere. Failure locates an observed disagreement but cannot alone distinguish oracle and implementation defects.

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

Epoch2026-10-30: original implementation and independent fixtures. Prospective relation: Finite executable observation: For all u64 n and nonzero m return F(n) mod m; F(0)=0,F(1)=1. m=0 returns ZeroModulus before arithmetic. Scan index bits preserving consecutive residues; use c=a(2b-a),d=a^2+b^2 mod m. Reference: Knuth, The Art of Computer Programming, volume1, section1.2.8. Assertions check independent arithmetic oracles, boundaries and identities; passing fixtures do not establish untested behavior.

### Undated · evidence

Source SHA-256: 93cfa692c659b7a0c88ed805f16cccae6271bbaff1de535a8035c063bfcae2f9.

- debug-1-fibonacci.log: b25fa6effa9a946a2734d7e40a3a501538ce58d830b08a93d9d9063a7c734168

- debug-2-fibonacci.log: bca61daa9d2d6df6276c03bf11cd283ae6aa76bfab8ee02aad017e8ae0a5c994

- release-1-fibonacci.log: 6f8db96d2710a8b290bf3ceab53c8a40e2cc83829cf0d88e7bd9e51008fea725

- release-2-fibonacci.log: 6f8db96d2710a8b290bf3ceab53c8a40e2cc83829cf0d88e7bd9e51008fea725

- mutant-fibonacci.log: f3e531d2d06cf700d87de74eebd1df5056ee19fee8a239271d752dbdb5378b7d
