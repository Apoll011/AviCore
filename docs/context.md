# context

```Namespace: global/context```

<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> get </h2>

```rust,ignore
fn get(key: String) -> ?
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
Gets a value from the skill's persistent context
</div>
<div group="get" id="get-Arguments" class="tabcontent"  style="display: none;" >
* `key` - The key of the value to retrieve
</div>
<div group="get" id="get-Returns" class="tabcontent"  style="display: none;" >
The value associated with the key, or UNIT if not found
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> has </h2>

```rust,ignore
fn has(key: String) -> bool
```

<div>
<div class="tab">
<button group="has" id="link-has-Description"  class="tablinks active" 
    onclick="openTab(event, 'has', 'Description')">
Description
</button>
<button group="has" id="link-has-Arguments"  class="tablinks" 
    onclick="openTab(event, 'has', 'Arguments')">
Arguments
</button>
<button group="has" id="link-has-Returns"  class="tablinks" 
    onclick="openTab(event, 'has', 'Returns')">
Returns
</button>
</div>

<div group="has" id="has-Description" class="tabcontent"  style="display: block;" >
Checks if a key exists in the skill's persistent context
</div>
<div group="has" id="has-Arguments" class="tabcontent"  style="display: none;" >
* `key` - The key to check
</div>
<div group="has" id="has-Returns" class="tabcontent"  style="display: none;" >
True if the key exists, false otherwise
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> remove </h2>

```rust,ignore
fn remove(key: String)
```

<div>
<div class="tab">
<button group="remove" id="link-remove-Description"  class="tablinks active" 
    onclick="openTab(event, 'remove', 'Description')">
Description
</button>
<button group="remove" id="link-remove-Arguments"  class="tablinks" 
    onclick="openTab(event, 'remove', 'Arguments')">
Arguments
</button>
<button group="remove" id="link-remove-Returns"  class="tablinks" 
    onclick="openTab(event, 'remove', 'Returns')">
Returns
</button>
</div>

<div group="remove" id="remove-Description" class="tabcontent"  style="display: block;" >
Removes a value from the skill's persistent context
</div>
<div group="remove" id="remove-Arguments" class="tabcontent"  style="display: none;" >
* `key` - The key of the value to remove
</div>
<div group="remove" id="remove-Returns" class="tabcontent"  style="display: none;" >
Nothing
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> set </h2>

```rust,ignore
fn set(key: String, value: ?, ttl: u64, persist: bool)
```

<div>
<div class="tab">
<button group="set" id="link-set-Description"  class="tablinks active" 
    onclick="openTab(event, 'set', 'Description')">
Description
</button>
<button group="set" id="link-set-Arguments"  class="tablinks" 
    onclick="openTab(event, 'set', 'Arguments')">
Arguments
</button>
<button group="set" id="link-set-Returns"  class="tablinks" 
    onclick="openTab(event, 'set', 'Returns')">
Returns
</button>
</div>

<div group="set" id="set-Description" class="tabcontent"  style="display: block;" >
Sets a value in the skill's persistent context
</div>
<div group="set" id="set-Arguments" class="tabcontent"  style="display: none;" >
* `key` - The key to set
* `value` - The value to store
* `ttl` - Time to live in seconds (0 for no TTL)
* `persist` - Whether to persist the value across sessions
</div>
<div group="set" id="set-Returns" class="tabcontent"  style="display: none;" >
Nothing
</div>

</div>
</div>
</br>
