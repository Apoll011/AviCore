# rand

```Namespace: global/rand```

<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> rand </h2>

```rust,ignore
fn rand() -> int
fn rand(range: RangeInclusive<int>) -> int
fn rand(range: Range<int>) -> int
fn rand(start: int, end: int) -> int
```

<div>
<div class="tab">
<button group="rand" id="link-rand-Description"  class="tablinks active" 
    onclick="openTab(event, 'rand', 'Description')">
Description
</button>
<button group="rand" id="link-rand-Example"  class="tablinks" 
    onclick="openTab(event, 'rand', 'Example')">
Example
</button>
</div>

<div group="rand" id="rand-Description" class="tabcontent"  style="display: block;" >
Generate a random integer number.
</div>
<div group="rand" id="rand-Example" class="tabcontent"  style="display: none;" >

```rhai
let number = rand();

print(`I'll give you a random number: ${number}`);
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> rand_bool </h2>

```rust,ignore
fn rand_bool() -> bool
fn rand_bool(probability: float) -> bool
```

<div>
<div class="tab">
<button group="rand_bool" id="link-rand_bool-Description"  class="tablinks active" 
    onclick="openTab(event, 'rand_bool', 'Description')">
Description
</button>
<button group="rand_bool" id="link-rand_bool-Example"  class="tablinks" 
    onclick="openTab(event, 'rand_bool', 'Example')">
Example
</button>
</div>

<div group="rand_bool" id="rand_bool-Description" class="tabcontent"  style="display: block;" >
Generate a random boolean value.
</div>
<div group="rand_bool" id="rand_bool-Example" class="tabcontent"  style="display: none;" >

```rhai
let decision = rand_bool();

if decision {
    print("You hit the Jackpot!")
}
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> rand_float </h2>

```rust,ignore
fn rand_float() -> float
fn rand_float(start: float, end: float) -> float
```

<div>
<div class="tab">
<button group="rand_float" id="link-rand_float-Description"  class="tablinks active" 
    onclick="openTab(event, 'rand_float', 'Description')">
Description
</button>
<button group="rand_float" id="link-rand_float-Example"  class="tablinks" 
    onclick="openTab(event, 'rand_float', 'Example')">
Example
</button>
</div>

<div group="rand_float" id="rand_float-Description" class="tabcontent"  style="display: block;" >
Generate a random floating-point number between `0.0` and `1.0` (exclusive).
Requires the `float` feature.

`1.0` is _excluded_ from the possibilities.
</div>
<div group="rand_float" id="rand_float-Example" class="tabcontent"  style="display: none;" >

```rhai
let number = rand_float();

print(`I'll give you a random number between 0 and 1: ${number}`);
```
</div>

</div>
</div>
</br>
