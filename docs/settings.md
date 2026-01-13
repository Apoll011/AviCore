# settings

```Namespace: global/settings```

<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> full </h2>

```rust,ignore
fn full(name: String) -> Option<SettingNamed>
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
Gets the full setting object including metadata
</div>
<div group="full" id="full-Arguments" class="tabcontent"  style="display: none;" >
* `name` - The name of the setting
</div>
<div group="full" id="full-Returns" class="tabcontent"  style="display: none;" >
The full setting object, or UNIT if not found
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> get </h2>

```rust,ignore
fn get(name: String) -> Option<?>
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
Gets a setting value for the current skill
</div>
<div group="get" id="get-Arguments" class="tabcontent"  style="display: none;" >
* `name` - The name of the setting
</div>
<div group="get" id="get-Returns" class="tabcontent"  style="display: none;" >
The setting value, or default if not set
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> has </h2>

```rust,ignore
fn has(name: String) -> bool
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
Checks if a setting exists for the current skill
</div>
<div group="has" id="has-Arguments" class="tabcontent"  style="display: none;" >
* `name` - The name of the setting
</div>
<div group="has" id="has-Returns" class="tabcontent"  style="display: none;" >
True if the setting exists, false otherwise
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> list </h2>

```rust,ignore
fn list() -> HashMap<String,Setting>
```

<div>
<div class="tab">
<button group="list" id="link-list-Description"  class="tablinks active" 
    onclick="openTab(event, 'list', 'Description')">
Description
</button>
<button group="list" id="link-list-Returns"  class="tablinks" 
    onclick="openTab(event, 'list', 'Returns')">
Returns
</button>
</div>

<div group="list" id="list-Description" class="tabcontent"  style="display: block;" >
Lists all settings available for the current skill
</div>
<div group="list" id="list-Returns" class="tabcontent"  style="display: none;" >
A list of setting names
</div>

</div>
</div>
</br>
