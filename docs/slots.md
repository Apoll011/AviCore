# slots

```Namespace: global/slots```

<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> assert_equal </h2>

```rust,ignore
fn assert_equal(intent: Intent, name: String, val: ?) -> bool
```

<div>
<div class="tab">
<button group="assert_equal" id="link-assert_equal-Description"  class="tablinks active" 
    onclick="openTab(event, 'assert_equal', 'Description')">
Description
</button>
<button group="assert_equal" id="link-assert_equal-Arguments"  class="tablinks" 
    onclick="openTab(event, 'assert_equal', 'Arguments')">
Arguments
</button>
<button group="assert_equal" id="link-assert_equal-Returns"  class="tablinks" 
    onclick="openTab(event, 'assert_equal', 'Returns')">
Returns
</button>
</div>

<div group="assert_equal" id="assert_equal-Description" class="tabcontent"  style="display: block;" >
Asserts that a slot's value is equal to a given value
</div>
<div group="assert_equal" id="assert_equal-Arguments" class="tabcontent"  style="display: none;" >
* `intent` - The intent to check
* `name` - The name of the slot
* `val` - The value to compare against
</div>
<div group="assert_equal" id="assert_equal-Returns" class="tabcontent"  style="display: none;" >
True if the slot value is equal, false otherwise
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> assert_in </h2>

```rust,ignore
fn assert_in(intent: Intent, name: String, list: ?) -> bool
```

<div>
<div class="tab">
<button group="assert_in" id="link-assert_in-Description"  class="tablinks active" 
    onclick="openTab(event, 'assert_in', 'Description')">
Description
</button>
<button group="assert_in" id="link-assert_in-Arguments"  class="tablinks" 
    onclick="openTab(event, 'assert_in', 'Arguments')">
Arguments
</button>
<button group="assert_in" id="link-assert_in-Returns"  class="tablinks" 
    onclick="openTab(event, 'assert_in', 'Returns')">
Returns
</button>
</div>

<div group="assert_in" id="assert_in-Description" class="tabcontent"  style="display: block;" >
Asserts that a slot's value is in a given list or matches a value
</div>
<div group="assert_in" id="assert_in-Arguments" class="tabcontent"  style="display: none;" >
* `intent` - The intent to check
* `name` - The name of the slot
* `list` - The list of values or a single value to check against
</div>
<div group="assert_in" id="assert_in-Returns" class="tabcontent"  style="display: none;" >
True if the slot value is in the list or matches the value, false otherwise
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> assert_in_dict </h2>

```rust,ignore
fn assert_in_dict(intent: Intent, name: String, dict: ?) -> bool
```

<div>
<div class="tab">
<button group="assert_in_dict" id="link-assert_in_dict-Description"  class="tablinks active" 
    onclick="openTab(event, 'assert_in_dict', 'Description')">
Description
</button>
<button group="assert_in_dict" id="link-assert_in_dict-Arguments"  class="tablinks" 
    onclick="openTab(event, 'assert_in_dict', 'Arguments')">
Arguments
</button>
<button group="assert_in_dict" id="link-assert_in_dict-Returns"  class="tablinks" 
    onclick="openTab(event, 'assert_in_dict', 'Returns')">
Returns
</button>
</div>

<div group="assert_in_dict" id="assert_in_dict-Description" class="tabcontent"  style="display: block;" >
Asserts that a slot's string value is a key in a given map
</div>
<div group="assert_in_dict" id="assert_in_dict-Arguments" class="tabcontent"  style="display: none;" >
* `intent` - The intent to check
* `name` - The name of the slot
* `dict` - The map to check for the key
</div>
<div group="assert_in_dict" id="assert_in_dict-Returns" class="tabcontent"  style="display: none;" >
True if the slot's string value is a key in the map, false otherwise
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> exists </h2>

```rust,ignore
fn exists(intent: Intent, name: String) -> bool
```

<div>
<div class="tab">
<button group="exists" id="link-exists-Description"  class="tablinks active" 
    onclick="openTab(event, 'exists', 'Description')">
Description
</button>
<button group="exists" id="link-exists-Arguments"  class="tablinks" 
    onclick="openTab(event, 'exists', 'Arguments')">
Arguments
</button>
<button group="exists" id="link-exists-Returns"  class="tablinks" 
    onclick="openTab(event, 'exists', 'Returns')">
Returns
</button>
</div>

<div group="exists" id="exists-Description" class="tabcontent"  style="display: block;" >
Checks if a slot exists in the current intent
</div>
<div group="exists" id="exists-Arguments" class="tabcontent"  style="display: none;" >
* `intent` - The intent to check
* `name` - The name of the slot
</div>
<div group="exists" id="exists-Returns" class="tabcontent"  style="display: none;" >
True if the slot exists, false otherwise
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> full </h2>

```rust,ignore
fn full(intent: Intent, name: String) -> ?
```

<div>
<div class="tab">
<button group="full" id="link-full-Description"  class="tablinks active" 
    onclick="openTab(event, 'full', 'Description')">
Description
</button>
<button group="full" id="link-full-Arguments"  class="tablinks" 
    onclick="openTab(event, 'full', 'Arguments')">
Arguments
</button>
<button group="full" id="link-full-Returns"  class="tablinks" 
    onclick="openTab(event, 'full', 'Returns')">
Returns
</button>
</div>

<div group="full" id="full-Description" class="tabcontent"  style="display: block;" >
Gets the full slot object from the current intent
</div>
<div group="full" id="full-Arguments" class="tabcontent"  style="display: none;" >
* `intent` - The intent to check
* `name` - The name of the slot
</div>
<div group="full" id="full-Returns" class="tabcontent"  style="display: none;" >
The full slot object, or UNIT if the slot is missing
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> get </h2>

```rust,ignore
fn get(intent: Intent, name: String) -> ?
```

<div>
<div class="tab">
<button group="get" id="link-get-Description"  class="tablinks active" 
    onclick="openTab(event, 'get', 'Description')">
Description
</button>
<button group="get" id="link-get-Arguments"  class="tablinks" 
    onclick="openTab(event, 'get', 'Arguments')">
Arguments
</button>
<button group="get" id="link-get-Returns"  class="tablinks" 
    onclick="openTab(event, 'get', 'Returns')">
Returns
</button>
</div>

<div group="get" id="get-Description" class="tabcontent"  style="display: block;" >
Gets the value of a slot from the current intent
</div>
<div group="get" id="get-Arguments" class="tabcontent"  style="display: none;" >
* `intent` - The intent to check
* `name` - The name of the slot
</div>
<div group="get" id="get-Returns" class="tabcontent"  style="display: none;" >
The value of the slot as a Rhai object, or UNIT if the slot is missing
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> get_raw </h2>

```rust,ignore
fn get_raw(intent: Intent, name: String) -> ?
```

<div>
<div class="tab">
<button group="get_raw" id="link-get_raw-Description"  class="tablinks active" 
    onclick="openTab(event, 'get_raw', 'Description')">
Description
</button>
<button group="get_raw" id="link-get_raw-Arguments"  class="tablinks" 
    onclick="openTab(event, 'get_raw', 'Arguments')">
Arguments
</button>
<button group="get_raw" id="link-get_raw-Returns"  class="tablinks" 
    onclick="openTab(event, 'get_raw', 'Returns')">
Returns
</button>
</div>

<div group="get_raw" id="get_raw-Description" class="tabcontent"  style="display: block;" >
Gets the raw text value of a slot from the current intent
</div>
<div group="get_raw" id="get_raw-Arguments" class="tabcontent"  style="display: none;" >
* `intent` - The intent to check
* `name` - The name of the slot
</div>
<div group="get_raw" id="get_raw-Returns" class="tabcontent"  style="display: none;" >
The raw text value of the slot, or UNIT if the slot is missing
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> require </h2>

```rust,ignore
fn require(intent: Intent, name: String)
```

<div>
<div class="tab">
<button group="require" id="link-require-Description"  class="tablinks active" 
    onclick="openTab(event, 'require', 'Description')">
Description
</button>
<button group="require" id="link-require-Arguments"  class="tablinks" 
    onclick="openTab(event, 'require', 'Arguments')">
Arguments
</button>
<button group="require" id="link-require-Returns"  class="tablinks" 
    onclick="openTab(event, 'require', 'Returns')">
Returns
</button>
</div>

<div group="require" id="require-Description" class="tabcontent"  style="display: block;" >
Asserts that a slot exists in the current intent
</div>
<div group="require" id="require-Arguments" class="tabcontent"  style="display: none;" >
* `intent` - The intent to check
* `name` - The name of the slot
</div>
<div group="require" id="require-Returns" class="tabcontent"  style="display: none;" >
Nothing or throws an error if the slot is missing
</div>

</div>
</div>
</br>
