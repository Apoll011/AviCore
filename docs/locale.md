# locale

```Namespace: global/locale```

<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> current </h2>

```rust,ignore
fn current() -> String
```

<div>
<div class="tab">
<button group="current" id="link-current-Description"  class="tablinks active" 
    onclick="openTab(event, 'current', 'Description')">
Description
</button>
<button group="current" id="link-current-Returns"  class="tablinks" 
    onclick="openTab(event, 'current', 'Returns')">
Returns
</button>
</div>

<div group="current" id="current-Description" class="tabcontent"  style="display: block;" >
Gets the current language code
</div>
<div group="current" id="current-Returns" class="tabcontent"  style="display: none;" >
The current language code (e.g., 'en-US')
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> get </h2>

```rust,ignore
fn get(id: String) -> String
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
Gets a translation for a given ID in the current locale
</div>
<div group="get" id="get-Arguments" class="tabcontent"  style="display: none;" >
* `id` - The ID of the translation to retrieve
</div>
<div group="get" id="get-Returns" class="tabcontent"  style="display: none;" >
The translation string if found, or None if not found
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> get_fmt </h2>

```rust,ignore
fn get_fmt(id: String, params: Map) -> String
```

<div>
<div class="tab">
<button group="get_fmt" id="link-get_fmt-Description"  class="tablinks active" 
    onclick="openTab(event, 'get_fmt', 'Description')">
Description
</button>
<button group="get_fmt" id="link-get_fmt-Arguments"  class="tablinks" 
    onclick="openTab(event, 'get_fmt', 'Arguments')">
Arguments
</button>
<button group="get_fmt" id="link-get_fmt-Returns"  class="tablinks" 
    onclick="openTab(event, 'get_fmt', 'Returns')">
Returns
</button>
</div>

<div group="get_fmt" id="get_fmt-Description" class="tabcontent"  style="display: block;" >
Gets a formatted translation for a given ID in the current locale
</div>
<div group="get_fmt" id="get_fmt-Arguments" class="tabcontent"  style="display: none;" >
* `id` - The ID of the translation to retrieve
* `params` - A map of parameters to interpolate into the translation
</div>
<div group="get_fmt" id="get_fmt-Returns" class="tabcontent"  style="display: none;" >
The formatted translation string if found, or UNIT if not found
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> has </h2>

```rust,ignore
fn has(id: String) -> bool
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
Checks if a translation exists for a given ID
</div>
<div group="has" id="has-Arguments" class="tabcontent"  style="display: none;" >
* `id` - The ID of the translation to check
</div>
<div group="has" id="has-Returns" class="tabcontent"  style="display: none;" >
True if the translation exists, false otherwise
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> list </h2>

```rust,ignore
fn list(code: String) -> Value)>
```

<div>
<div class="tab">
<button group="list" id="link-list-Description"  class="tablinks active" 
    onclick="openTab(event, 'list', 'Description')">
Description
</button>
<button group="list" id="link-list-Arguments"  class="tablinks" 
    onclick="openTab(event, 'list', 'Arguments')">
Arguments
</button>
<button group="list" id="link-list-Returns"  class="tablinks" 
    onclick="openTab(event, 'list', 'Returns')">
Returns
</button>
</div>

<div group="list" id="list-Description" class="tabcontent"  style="display: block;" >
Lists all translations for a given locale code
</div>
<div group="list" id="list-Arguments" class="tabcontent"  style="display: none;" >
* `code` - The locale code (e.g., 'en-US')
</div>
<div group="list" id="list-Returns" class="tabcontent"  style="display: none;" >
A map of translations
</div>

</div>
</div>
</br>
