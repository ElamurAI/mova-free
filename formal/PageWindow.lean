/-
  The pager's window (`page_window` of the sites' internal pager, not in the public tree), written again in Lean as its specification.

  `pageWindow n cur` is the list a pager shows for `n` pages with page `cur` (0-based) current: `some k` is page k,
  `none` a gap «…». The function is the Rust one step for step (the same loop, with fuel instead of `while`).
  The theorems are proved by evaluation on every `n ≤ 30` and every current page (`decide`): no `sorry`, no axiom.
  `#eval table 60` prints every window up to 60 pages; the Rust test compares its own windows with that table.
-/

def loop (n cur : Nat) (leftGap rightGap : Bool) : Nat → Nat → List (Option Nat)
  | 0, _ => []
  | fuel + 1, i =>
    if i > n then []
    else if leftGap && i == 2 then none :: loop n cur leftGap rightGap fuel (cur - 1)
    else if rightGap && i == cur + 2 then none :: loop n cur leftGap rightGap fuel n
    else some (i - 1) :: loop n cur leftGap rightGap fuel (i + 1)

/-- The plain window: the first, the current one with one on each side, the last. -/
def baseWindow (n cur : Nat) : List (Option Nat) :=
  if n = 0 then [] else
  let c := min cur (n - 1) + 1
  loop n c (decide (c > 4)) (decide (c + 2 < n - 1)) (n + 1) 1

/-- Insertion into a sorted list without repeats, and the sort it makes (structural, so the kernel can evaluate
    it for `decide`). -/
def insert1 (x : Nat) : List Nat → List Nat
  | [] => [x]
  | y :: t => if x < y then x :: y :: t else if x == y then y :: t else y :: insert1 x t

def isort (l : List Nat) : List Nat := l.foldr insert1 []

/-- The pages shown, 1-based and sorted: the plain window's, and for each scale 10, 100, 1000 under `n` the nearest
    multiple strictly below and the one above the current page. -/
def shown (n cur : Nat) : List Nat :=
  let c := min cur (n - 1) + 1
  let base := (baseWindow n cur).filterMap id |>.map (· + 1)
  let near := ([10, 100, 1000].filter (· < n)).flatMap fun s => [(c - 1) / s * s, (c / s + 1) * s]
  let all := (base ++ near).filter (fun x => 1 ≤ x && x ≤ n)
  isort all

/-- The shown pages with the gaps: a gap where two pages or more are skipped, the page itself where one is. -/
def fill : Nat → List Nat → List (Option Nat)
  | _, [] => []
  | last, k :: t =>
    (if last > 0 && k == last + 2 then [some last] else if last > 0 && k > last + 2 then [none] else [])
      ++ some (k - 1) :: fill k t

def pageWindow (n cur : Nat) : List (Option Nat) := if n = 0 then [] else fill 0 (shown n cur)

/-- The pages a window shows, in order. -/
def pages (w : List (Option Nat)) : List Nat := w.filterMap id

/-- Strictly increasing. -/
def increasing : List Nat → Bool
  | a :: b :: t => a < b && increasing (b :: t)
  | _ => true

/-- Between two neighbours: a page right after a page differs by one; a gap stands only where two or more pages are
    skipped; never two gaps together; the window starts and ends with a page. -/
def wellGapped : List (Option Nat) → Bool
  | some a :: some b :: t => b == a + 1 && wellGapped (some b :: t)
  | some a :: none :: some b :: t => b ≥ a + 3 && wellGapped (some b :: t)
  | [some _] => true
  | [] => true
  | _ => false

def ok (n cur : Nat) : Bool :=
  let w := pageWindow n cur
  let c := min cur (n - 1)
  (pages w).length ≤ 11 && (pages w).contains 0 && (pages w).contains (n - 1) && (pages w).contains c
    && increasing (pages w) && wellGapped w

/-- Every window up to 30 pages, every current page: at most eleven pages, the first, the last and the current page
    shown, the pages in order, a gap only over two pages or more. -/
theorem window_ok : ∀ n, n ≤ 30 → n ≥ 1 → ∀ cur, cur < n → ok n cur = true := by
  decide

/-- The deepest page from page 1 when one follows the windows (breadth first). -/
def maxDepth (n : Nat) : Nat := Id.run do
  let mut depth : Array Nat := Array.replicate (n + 1) 0
  let mut seen : Array Bool := (Array.replicate (n + 1) false).set! 0 true
  let mut frontier : List Nat := [0]
  let mut d := 0
  while !frontier.isEmpty do
    d := d + 1
    let mut next : List Nat := []
    for u in frontier do
      for v in (pageWindow n u).filterMap id do
        if !seen[v]! then
          seen := seen.set! v true
          depth := depth.set! v d
          next := v :: next
    frontier := next
  return depth.foldl max 0

/-- The table the Rust test reads: «n cur: items», a gap as «-», for every window up to 60 pages and the long lists
    of 99, 100, 101, 109, 999, 1000 and 1001 pages; then «#depth n d» for every n up to 1000. -/
def table : String :=
  let ns := List.range 61 ++ [99, 100, 101, 109, 999, 1000, 1001]
  let rows := ns.flatMap fun n => (List.range n).map fun cur =>
      s!"{n} {cur}: " ++ String.intercalate " " ((pageWindow n cur).map fun x => match x with | some k => toString k | none => "-")
  let depths := (List.range 1000).map fun i => s!"#depth {i + 1} {maxDepth (i + 1)}"
  String.intercalate "\n" (rows ++ depths)

#eval IO.println table
