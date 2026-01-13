# array

```Namespace: global/rand/array```

<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> sample </h2>

```rust,ignore
fn sample(array: Array) -> ?
fn sample(array: Array, amount: int) -> Array
```

<div>
<div class="tab">
<button group="sample" id="link-sample-Description"  class="tablinks active" 
    onclick="openTab(event, 'sample', 'Description')">
Description
</button>
<button group="sample" id="link-sample-Example"  class="tablinks" 
    onclick="openTab(event, 'sample', 'Example')">
Example
</button>
</div>

<div group="sample" id="sample-Description" class="tabcontent"  style="display: block;" >
Copy a random element from the array and return it.
Requires the `array` feature.
</div>
<div group="sample" id="sample-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [1, 2, 3, 4, 5];

let number = x.sample();

print(`I'll give you a random number between 1 and 5: ${number}`);
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> shuffle </h2>

```rust,ignore
fn shuffle(array: Array)
```

<div>
<div class="tab">
<button group="shuffle" id="link-shuffle-Description"  class="tablinks active" 
    onclick="openTab(event, 'shuffle', 'Description')">
Description
</button>
<button group="shuffle" id="link-shuffle-Example"  class="tablinks" 
    onclick="openTab(event, 'shuffle', 'Example')">
Example
</button>
</div>

<div group="shuffle" id="shuffle-Description" class="tabcontent"  style="display: block;" >
Shuffle the elements in the array.
Requires the `array` feature.
</div>
<div group="shuffle" id="shuffle-Example" class="tabcontent"  style="display: none;" >

```rhai
let x = [1, 2, 3, 4, 5];

x.shuffle();    // shuffle the elements inside the array
```
</div>

</div>
</div>
</br>
