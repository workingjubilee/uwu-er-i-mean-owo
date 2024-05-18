This repo contains code that induces subpar diagnostic errors described in https://github.com/rust-lang/rust/issues/125260

## rust version

```
rustc 1.80.0-nightly (ab14f944a 2024-05-13)
binary: rustc
commit-hash: ab14f944afe4234db378ced3801e637eae6c0f30
commit-date: 2024-05-13
host: x86_64-unknown-linux-gnu
release: 1.80.0-nightly
LLVM version: 18.1.4
```

## errors

```
  
    Checking whats-this v0.1.0 (/home/jubilee/rust/whats-this)
error[E0407]: method `uwu` is not a member of trait `WhatsThis`
  --> src/main.rs:14:13
   |
14 | /             fn uwu() -> Self {
15 | |                 <$this as Default>::default()
16 | |             }
   | |_____________^ not a member of trait `WhatsThis`
   |
  ::: src/ty0.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty0);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0407]: method `uwu` is not a member of trait `WhatsThis`
  --> src/main.rs:14:13
   |
14 | /             fn uwu() -> Self {
15 | |                 <$this as Default>::default()
16 | |             }
   | |_____________^ not a member of trait `WhatsThis`
   |
  ::: src/ty1.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty1);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0407]: method `uwu` is not a member of trait `WhatsThis`
  --> src/main.rs:14:13
   |
14 | /             fn uwu() -> Self {
15 | |                 <$this as Default>::default()
16 | |             }
   | |_____________^ not a member of trait `WhatsThis`
   |
  ::: src/ty2.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty2);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0407]: method `uwu` is not a member of trait `WhatsThis`
  --> src/main.rs:14:13
   |
14 | /             fn uwu() -> Self {
15 | |                 <$this as Default>::default()
16 | |             }
   | |_____________^ not a member of trait `WhatsThis`
   |
  ::: src/ty3.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty3);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0407]: method `uwu` is not a member of trait `WhatsThis`
  --> src/main.rs:14:13
   |
14 | /             fn uwu() -> Self {
15 | |                 <$this as Default>::default()
16 | |             }
   | |_____________^ not a member of trait `WhatsThis`
   |
  ::: src/ty4.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty4);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0407]: method `uwu` is not a member of trait `WhatsThis`
  --> src/main.rs:14:13
   |
14 | /             fn uwu() -> Self {
15 | |                 <$this as Default>::default()
16 | |             }
   | |_____________^ not a member of trait `WhatsThis`
   |
  ::: src/ty5.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty5);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0407]: method `uwu` is not a member of trait `WhatsThis`
  --> src/main.rs:14:13
   |
14 | /             fn uwu() -> Self {
15 | |                 <$this as Default>::default()
16 | |             }
   | |_____________^ not a member of trait `WhatsThis`
   |
  ::: src/ty6.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty6);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0407]: method `uwu` is not a member of trait `WhatsThis`
  --> src/main.rs:14:13
   |
14 | /             fn uwu() -> Self {
15 | |                 <$this as Default>::default()
16 | |             }
   | |_____________^ not a member of trait `WhatsThis`
   |
  ::: src/ty7.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty7);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0407]: method `uwu` is not a member of trait `WhatsThis`
  --> src/main.rs:14:13
   |
14 | /             fn uwu() -> Self {
15 | |                 <$this as Default>::default()
16 | |             }
   | |_____________^ not a member of trait `WhatsThis`
   |
  ::: src/ty8.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty8);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0407]: method `uwu` is not a member of trait `WhatsThis`
  --> src/main.rs:14:13
   |
14 | /             fn uwu() -> Self {
15 | |                 <$this as Default>::default()
16 | |             }
   | |_____________^ not a member of trait `WhatsThis`
   |
  ::: src/ty9.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty9);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: `#[macro_use]` only has an effect on `extern crate` and modules
 --> src/main.rs:9:1
  |
9 | #[macro_use]
  | ^^^^^^^^^^^^
  |
  = note: `#[warn(unused_attributes)]` on by default

error[E0046]: not all trait items implemented, missing: `owo`
  --> src/main.rs:13:9
   |
13 |           impl WhatsThis for $this {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^ missing `owo` in implementation
...
33 |       fn owo() -> Self;
   |       ----------------- `owo` from trait
   |
  ::: src/ty0.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty0);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `owo`
  --> src/main.rs:13:9
   |
13 |           impl WhatsThis for $this {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^ missing `owo` in implementation
...
33 |       fn owo() -> Self;
   |       ----------------- `owo` from trait
   |
  ::: src/ty1.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty1);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `owo`
  --> src/main.rs:13:9
   |
13 |           impl WhatsThis for $this {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^ missing `owo` in implementation
...
33 |       fn owo() -> Self;
   |       ----------------- `owo` from trait
   |
  ::: src/ty2.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty2);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `owo`
  --> src/main.rs:13:9
   |
13 |           impl WhatsThis for $this {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^ missing `owo` in implementation
...
33 |       fn owo() -> Self;
   |       ----------------- `owo` from trait
   |
  ::: src/ty3.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty3);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `owo`
  --> src/main.rs:13:9
   |
13 |           impl WhatsThis for $this {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^ missing `owo` in implementation
...
33 |       fn owo() -> Self;
   |       ----------------- `owo` from trait
   |
  ::: src/ty4.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty4);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `owo`
  --> src/main.rs:13:9
   |
13 |           impl WhatsThis for $this {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^ missing `owo` in implementation
...
33 |       fn owo() -> Self;
   |       ----------------- `owo` from trait
   |
  ::: src/ty5.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty5);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `owo`
  --> src/main.rs:13:9
   |
13 |           impl WhatsThis for $this {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^ missing `owo` in implementation
...
33 |       fn owo() -> Self;
   |       ----------------- `owo` from trait
   |
  ::: src/ty6.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty6);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `owo`
  --> src/main.rs:13:9
   |
13 |           impl WhatsThis for $this {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^ missing `owo` in implementation
...
33 |       fn owo() -> Self;
   |       ----------------- `owo` from trait
   |
  ::: src/ty7.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty7);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `owo`
  --> src/main.rs:13:9
   |
13 |           impl WhatsThis for $this {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^ missing `owo` in implementation
...
33 |       fn owo() -> Self;
   |       ----------------- `owo` from trait
   |
  ::: src/ty8.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty8);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0046]: not all trait items implemented, missing: `owo`
  --> src/main.rs:13:9
   |
13 |           impl WhatsThis for $this {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^ missing `owo` in implementation
...
33 |       fn owo() -> Self;
   |       ----------------- `owo` from trait
   |
  ::: src/ty9.rs:4:1
   |
4  | / uwuify
5  | | !
6  | | (Ty9);
   | |_____- in this macro invocation
   |
   = note: this error originates in the macro `uwuify` (in Nightly builds, run with -Z macro-backtrace for more info)

Some errors have detailed explanations: E0046, E0407.
For more information about an error, try `rustc --explain E0046`.
warning: `whats-this` (bin "whats-this") generated 1 warning
error: could not compile `whats-this` (bin "whats-this") due to 20 previous errors; 1 warning emitted

```
