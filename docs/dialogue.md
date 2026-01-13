# dialogue

```Namespace: global/dialogue```

<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> any_validator </h2>

```rust,ignore
fn any_validator() -> AnyValidator
```

<div>
<div class="tab">
<button group="any_validator" id="link-any_validator-Description"  class="tablinks active" 
    onclick="openTab(event, 'any_validator', 'Description')">
Description
</button>
<button group="any_validator" id="link-any_validator-Returns"  class="tablinks" 
    onclick="openTab(event, 'any_validator', 'Returns')">
Returns
</button>
</div>

<div group="any_validator" id="any_validator-Description" class="tabcontent"  style="display: block;" >
Creates a validator that accepts any input
</div>
<div group="any_validator" id="any_validator-Returns" class="tabcontent"  style="display: none;" >
An AnyValidator object
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> bool_validator </h2>

```rust,ignore
fn bool_validator(fuzzy: bool) -> BoolValidator
```

<div>
<div class="tab">
<button group="bool_validator" id="link-bool_validator-Description"  class="tablinks active" 
    onclick="openTab(event, 'bool_validator', 'Description')">
Description
</button>
<button group="bool_validator" id="link-bool_validator-Arguments"  class="tablinks" 
    onclick="openTab(event, 'bool_validator', 'Arguments')">
Arguments
</button>
<button group="bool_validator" id="link-bool_validator-Returns"  class="tablinks" 
    onclick="openTab(event, 'bool_validator', 'Returns')">
Returns
</button>
</div>

<div group="bool_validator" id="bool_validator-Description" class="tabcontent"  style="display: block;" >
Creates a validator that accepts boolean input (yes/no)
</div>
<div group="bool_validator" id="bool_validator-Arguments" class="tabcontent"  style="display: none;" >
* `fuzzy` - Whether to use fuzzy matching
</div>
<div group="bool_validator" id="bool_validator-Returns" class="tabcontent"  style="display: none;" >
A BoolValidator object
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> confirm </h2>

```rust,ignore
fn confirm(question_locale_id: String, handler: String)
```

<div>
<div class="tab">
<button group="confirm" id="link-confirm-Description"  class="tablinks active" 
    onclick="openTab(event, 'confirm', 'Description')">
Description
</button>
<button group="confirm" id="link-confirm-Arguments"  class="tablinks" 
    onclick="openTab(event, 'confirm', 'Arguments')">
Arguments
</button>
<button group="confirm" id="link-confirm-Returns"  class="tablinks" 
    onclick="openTab(event, 'confirm', 'Returns')">
Returns
</button>
</div>

<div group="confirm" id="confirm-Description" class="tabcontent"  style="display: block;" >
Asks the user a yes/no question and handles the response
</div>
<div group="confirm" id="confirm-Arguments" class="tabcontent"  style="display: none;" >
* `question_locale_id` - The locale ID of the question to ask
* `handler` - The name of the function to call with the result
</div>
<div group="confirm" id="confirm-Returns" class="tabcontent"  style="display: none;" >
Nothing
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> list_or_none_validator </h2>

```rust,ignore
fn list_or_none_validator(allowed_values: Vec<String>) -> ListOrNoneValidator
```

<div>
<div class="tab">
<button group="list_or_none_validator" id="link-list_or_none_validator-Description"  class="tablinks active" 
    onclick="openTab(event, 'list_or_none_validator', 'Description')">
Description
</button>
<button group="list_or_none_validator" id="link-list_or_none_validator-Arguments"  class="tablinks" 
    onclick="openTab(event, 'list_or_none_validator', 'Arguments')">
Arguments
</button>
<button group="list_or_none_validator" id="link-list_or_none_validator-Returns"  class="tablinks" 
    onclick="openTab(event, 'list_or_none_validator', 'Returns')">
Returns
</button>
</div>

<div group="list_or_none_validator" id="list_or_none_validator-Description" class="tabcontent"  style="display: block;" >
Creates a validator that accepts a list of items or nothing
</div>
<div group="list_or_none_validator" id="list_or_none_validator-Arguments" class="tabcontent"  style="display: none;" >
* `allowed_values` - A list of accepted string values
</div>
<div group="list_or_none_validator" id="list_or_none_validator-Returns" class="tabcontent"  style="display: none;" >
A ListOrNoneValidator object
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> listen </h2>

```rust,ignore
fn listen()
```

<div>
<div class="tab">
<button group="listen" id="link-listen-Description"  class="tablinks active" 
    onclick="openTab(event, 'listen', 'Description')">
Description
</button>
<button group="listen" id="link-listen-Returns"  class="tablinks" 
    onclick="openTab(event, 'listen', 'Returns')">
Returns
</button>
</div>

<div group="listen" id="listen-Description" class="tabcontent"  style="display: block;" >
Makes the device start listening for voice input
</div>
<div group="listen" id="listen-Returns" class="tabcontent"  style="display: none;" >
Nothing
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> mapped_validator </h2>

```rust,ignore
fn mapped_validator(map: Map) -> MappedValidator
```

<div>
<div class="tab">
<button group="mapped_validator" id="link-mapped_validator-Description"  class="tablinks active" 
    onclick="openTab(event, 'mapped_validator', 'Description')">
Description
</button>
<button group="mapped_validator" id="link-mapped_validator-Arguments"  class="tablinks" 
    onclick="openTab(event, 'mapped_validator', 'Arguments')">
Arguments
</button>
<button group="mapped_validator" id="link-mapped_validator-Returns"  class="tablinks" 
    onclick="openTab(event, 'mapped_validator', 'Returns')">
Returns
</button>
</div>

<div group="mapped_validator" id="mapped_validator-Description" class="tabcontent"  style="display: block;" >
Creates a validator that maps string input to values
</div>
<div group="mapped_validator" id="mapped_validator-Arguments" class="tabcontent"  style="display: none;" >
* `map` - A map of possible inputs to their string values
</div>
<div group="mapped_validator" id="mapped_validator-Returns" class="tabcontent"  style="display: none;" >
A MappedValidator object
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> on_reply </h2>

```rust,ignore
fn on_reply(handler: String, validator: ?)
```

<div>
<div class="tab">
<button group="on_reply" id="link-on_reply-Description"  class="tablinks active" 
    onclick="openTab(event, 'on_reply', 'Description')">
Description
</button>
<button group="on_reply" id="link-on_reply-Arguments"  class="tablinks" 
    onclick="openTab(event, 'on_reply', 'Arguments')">
Arguments
</button>
<button group="on_reply" id="link-on_reply-Returns"  class="tablinks" 
    onclick="openTab(event, 'on_reply', 'Returns')">
Returns
</button>
</div>

<div group="on_reply" id="on_reply-Description" class="tabcontent"  style="display: block;" >
Registers a handler for the next user response
</div>
<div group="on_reply" id="on_reply-Arguments" class="tabcontent"  style="display: none;" >
* `handler` - The name of the function to call with the response
* `validator` - The validator to use for the response
</div>
<div group="on_reply" id="on_reply-Returns" class="tabcontent"  style="display: none;" >
Nothing
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> optional_validator </h2>

```rust,ignore
fn optional_validator() -> OptionalValidator
```

<div>
<div class="tab">
<button group="optional_validator" id="link-optional_validator-Description"  class="tablinks active" 
    onclick="openTab(event, 'optional_validator', 'Description')">
Description
</button>
<button group="optional_validator" id="link-optional_validator-Returns"  class="tablinks" 
    onclick="openTab(event, 'optional_validator', 'Returns')">
Returns
</button>
</div>

<div group="optional_validator" id="optional_validator-Description" class="tabcontent"  style="display: block;" >
Creates a validator that makes another validator optional
</div>
<div group="optional_validator" id="optional_validator-Returns" class="tabcontent"  style="display: none;" >
An OptionalValidator object
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> repeat </h2>

```rust,ignore
fn repeat()
```

<div>
<div class="tab">
<button group="repeat" id="link-repeat-Description"  class="tablinks active" 
    onclick="openTab(event, 'repeat', 'Description')">
Description
</button>
<button group="repeat" id="link-repeat-Returns"  class="tablinks" 
    onclick="openTab(event, 'repeat', 'Returns')">
Returns
</button>
</div>

<div group="repeat" id="repeat-Description" class="tabcontent"  style="display: block;" >
Repeats the last thing spoken or heard
</div>
<div group="repeat" id="repeat-Returns" class="tabcontent"  style="display: none;" >
Nothing
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> request_attention </h2>

```rust,ignore
fn request_attention()
```

<div>
<div class="tab">
<button group="request_attention" id="link-request_attention-Description"  class="tablinks active" 
    onclick="openTab(event, 'request_attention', 'Description')">
Description
</button>
<button group="request_attention" id="link-request_attention-Returns"  class="tablinks" 
    onclick="openTab(event, 'request_attention', 'Returns')">
Returns
</button>
</div>

<div group="request_attention" id="request_attention-Description" class="tabcontent"  style="display: block;" >
Requests the user's attention and starts listening
</div>
<div group="request_attention" id="request_attention-Returns" class="tabcontent"  style="display: none;" >
Nothing
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> say </h2>

```rust,ignore
fn say(text: String)
```

<div>
<div class="tab">
<button group="say" id="link-say-Description"  class="tablinks active" 
    onclick="openTab(event, 'say', 'Description')">
Description
</button>
<button group="say" id="link-say-Arguments"  class="tablinks" 
    onclick="openTab(event, 'say', 'Arguments')">
Arguments
</button>
<button group="say" id="link-say-Returns"  class="tablinks" 
    onclick="openTab(event, 'say', 'Returns')">
Returns
</button>
</div>

<div group="say" id="say-Description" class="tabcontent"  style="display: block;" >
Speaks a given text
</div>
<div group="say" id="say-Arguments" class="tabcontent"  style="display: none;" >
* `text` - The text to speak
</div>
<div group="say" id="say-Returns" class="tabcontent"  style="display: none;" >
Nothing
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> say_once </h2>

```rust,ignore
fn say_once(text: String)
```

<div>
<div class="tab">
<button group="say_once" id="link-say_once-Description"  class="tablinks active" 
    onclick="openTab(event, 'say_once', 'Description')">
Description
</button>
<button group="say_once" id="link-say_once-Arguments"  class="tablinks" 
    onclick="openTab(event, 'say_once', 'Arguments')">
Arguments
</button>
<button group="say_once" id="link-say_once-Returns"  class="tablinks" 
    onclick="openTab(event, 'say_once', 'Returns')">
Returns
</button>
</div>

<div group="say_once" id="say_once-Description" class="tabcontent"  style="display: block;" >
Speaks a given text only once
</div>
<div group="say_once" id="say_once-Arguments" class="tabcontent"  style="display: none;" >
* `text` - The text to speak
</div>
<div group="say_once" id="say_once-Returns" class="tabcontent"  style="display: none;" >
Nothing
</div>

</div>
</div>
</br>
