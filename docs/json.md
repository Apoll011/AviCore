# json

```Namespace: global/json```

<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> parse </h2>

```rust,ignore
fn parse(json_str: String) -> ?
```

<div>
<div class="tab">
<button group="parse" id="link-parse-Description"  class="tablinks active" 
    onclick="openTab(event, 'parse', 'Description')">
Description
</button>
<button group="parse" id="link-parse-Arguments"  class="tablinks" 
    onclick="openTab(event, 'parse', 'Arguments')">
Arguments
</button>
<button group="parse" id="link-parse-Returns"  class="tablinks" 
    onclick="openTab(event, 'parse', 'Returns')">
Returns
</button>
</div>

<div group="parse" id="parse-Description" class="tabcontent"  style="display: block;" >
Parses a JSON string into a Rhai object
</div>
<div group="parse" id="parse-Arguments" class="tabcontent"  style="display: none;" >
* `json_str` - The JSON string to parse
</div>
<div group="parse" id="parse-Returns" class="tabcontent"  style="display: none;" >
A Rhai object representing the JSON data
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_json </h2>

```rust,ignore
fn to_json(value: ?) -> String
```

<div>
<div class="tab">
<button group="to_json" id="link-to_json-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_json', 'Description')">
Description
</button>
<button group="to_json" id="link-to_json-Arguments"  class="tablinks" 
    onclick="openTab(event, 'to_json', 'Arguments')">
Arguments
</button>
<button group="to_json" id="link-to_json-Returns"  class="tablinks" 
    onclick="openTab(event, 'to_json', 'Returns')">
Returns
</button>
</div>

<div group="to_json" id="to_json-Description" class="tabcontent"  style="display: block;" >
Converts a Rhai object into a pretty-printed JSON string
</div>
<div group="to_json" id="to_json-Arguments" class="tabcontent"  style="display: none;" >
* `value` - The Rhai object to convert
</div>
<div group="to_json" id="to_json-Returns" class="tabcontent"  style="display: none;" >
A JSON string
</div>

</div>
</div>
</br>
