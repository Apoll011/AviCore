# url

```Namespace: global/url```

<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> Url </h2>

```rust,ignore
fn Url(url: String) -> Url
```

<div>
<div class="tab">
<button group="Url" id="link-Url-Description"  class="tablinks active" 
    onclick="openTab(event, 'Url', 'Description')">
Description
</button>
<button group="Url" id="link-Url-Example"  class="tablinks" 
    onclick="openTab(event, 'Url', 'Example')">
Example
</button>
</div>

<div group="Url" id="Url-Description" class="tabcontent"  style="display: block;" >
Creates a new Url.
</div>
<div group="Url" id="Url-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/")
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Url.domain </h2>

```rust,ignore
get Url.domain -> String
```

<div>
<div class="tab">
<button group="Url.domain" id="link-Url.domain-Description"  class="tablinks active" 
    onclick="openTab(event, 'Url.domain', 'Description')">
Description
</button>
<button group="Url.domain" id="link-Url.domain-Example"  class="tablinks" 
    onclick="openTab(event, 'Url.domain', 'Example')">
Example
</button>
</div>

<div group="Url.domain" id="Url.domain-Description" class="tabcontent"  style="display: block;" >
Gets the Url domain.
</div>
<div group="Url.domain" id="Url.domain-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/")
let domain = url.domain // 'test.dev'
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Url.fragment </h2>

```rust,ignore
get Url.fragment -> String
set Url.fragment = String
set Url.fragment = Option<String>
```

<div>
<div class="tab">
<button group="Url.fragment" id="link-Url.fragment-Description"  class="tablinks active" 
    onclick="openTab(event, 'Url.fragment', 'Description')">
Description
</button>
<button group="Url.fragment" id="link-Url.fragment-Example"  class="tablinks" 
    onclick="openTab(event, 'Url.fragment', 'Example')">
Example
</button>
</div>

<div group="Url.fragment" id="Url.fragment-Description" class="tabcontent"  style="display: block;" >
Gets the Url fragment.
</div>
<div group="Url.fragment" id="Url.fragment-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/?#row=4")
let fragment = url.fragment // 'row=4'
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Url.hash </h2>

```rust,ignore
get Url.hash -> String
set Url.hash = Option<String>
set Url.hash = String
```

<div>
<div class="tab">
<button group="Url.hash" id="link-Url.hash-Description"  class="tablinks active" 
    onclick="openTab(event, 'Url.hash', 'Description')">
Description
</button>
<button group="Url.hash" id="link-Url.hash-Example"  class="tablinks" 
    onclick="openTab(event, 'Url.hash', 'Example')">
Example
</button>
</div>

<div group="Url.hash" id="Url.hash-Description" class="tabcontent"  style="display: block;" >
Gets the Url hash, alias of fragment.
</div>
<div group="Url.hash" id="Url.hash-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/?#row=4");
let hash = url.hash; // 'row=4'
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Url.href </h2>

```rust,ignore
get Url.href -> String
```

<div>
<div class="tab">
<button group="Url.href" id="link-Url.href-Description"  class="tablinks active" 
    onclick="openTab(event, 'Url.href', 'Description')">
Description
</button>
<button group="Url.href" id="link-Url.href-Example"  class="tablinks" 
    onclick="openTab(event, 'Url.href', 'Example')">
Example
</button>
</div>

<div group="Url.href" id="Url.href-Description" class="tabcontent"  style="display: block;" >
Gets the full Url, same as to_string().
</div>
<div group="Url.href" id="Url.href-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/")
let fullUrl = url.href // 'http://test.dev/'
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Url.path </h2>

```rust,ignore
get Url.path -> String
set Url.path = String
```

<div>
<div class="tab">
<button group="Url.path" id="link-Url.path-Description"  class="tablinks active" 
    onclick="openTab(event, 'Url.path', 'Description')">
Description
</button>
<button group="Url.path" id="link-Url.path-Example"  class="tablinks" 
    onclick="openTab(event, 'Url.path', 'Example')">
Example
</button>
</div>

<div group="Url.path" id="Url.path-Description" class="tabcontent"  style="display: block;" >
Gets the Url path.
</div>
<div group="Url.path" id="Url.path-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/path")
let path = url.path // '/path'
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Url.query </h2>

```rust,ignore
get Url.query -> String
set Url.query = String
set Url.query = Option<String>
```

<div>
<div class="tab">
<button group="Url.query" id="link-Url.query-Description"  class="tablinks active" 
    onclick="openTab(event, 'Url.query', 'Description')">
Description
</button>
<button group="Url.query" id="link-Url.query-Example"  class="tablinks" 
    onclick="openTab(event, 'Url.query', 'Example')">
Example
</button>
</div>

<div group="Url.query" id="Url.query-Description" class="tabcontent"  style="display: block;" >
Gets the Url query string.
</div>
<div group="Url.query" id="Url.query-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/?page=2")
let query = url.query // 'page=2'
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>get/set</code> Url.scheme </h2>

```rust,ignore
get Url.scheme -> String
set Url.scheme = String
```

<div>
<div class="tab">
<button group="Url.scheme" id="link-Url.scheme-Description"  class="tablinks active" 
    onclick="openTab(event, 'Url.scheme', 'Description')">
Description
</button>
<button group="Url.scheme" id="link-Url.scheme-Example"  class="tablinks" 
    onclick="openTab(event, 'Url.scheme', 'Example')">
Example
</button>
</div>

<div group="Url.scheme" id="Url.scheme-Description" class="tabcontent"  style="display: block;" >
Gets the Url scheme such as 'http' or 'https'.
</div>
<div group="Url.scheme" id="Url.scheme-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/")
let scheme = url.scheme // 'http'
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> query_clear </h2>

```rust,ignore
fn query_clear(url: Url)
```

<div>
<div class="tab">
<button group="query_clear" id="link-query_clear-Description"  class="tablinks active" 
    onclick="openTab(event, 'query_clear', 'Description')">
Description
</button>
<button group="query_clear" id="link-query_clear-Example"  class="tablinks" 
    onclick="openTab(event, 'query_clear', 'Example')">
Example
</button>
</div>

<div group="query_clear" id="query_clear-Description" class="tabcontent"  style="display: block;" >
Clear the query string.
</div>
<div group="query_clear" id="query_clear-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/?q=query&b=1");

url.query_clear();

url == "http://test.dev/"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> query_delete </h2>

```rust,ignore
fn query_delete(url: Url, key: String)
```

<div>
<div class="tab">
<button group="query_delete" id="link-query_delete-Description"  class="tablinks active" 
    onclick="openTab(event, 'query_delete', 'Description')">
Description
</button>
<button group="query_delete" id="link-query_delete-Example"  class="tablinks" 
    onclick="openTab(event, 'query_delete', 'Example')">
Example
</button>
</div>

<div group="query_delete" id="query_delete-Description" class="tabcontent"  style="display: block;" >
Delete a key from the query
</div>
<div group="query_delete" id="query_delete-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/?q=query&b=1");

url.query_delete("q"); // or
url.query_remove("q");

url == "http://test.dev/?b=1"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> query_get </h2>

```rust,ignore
fn query_get(url: Url, key: String) -> String
```

<div>
<div class="tab">
<button group="query_get" id="link-query_get-Description"  class="tablinks active" 
    onclick="openTab(event, 'query_get', 'Description')">
Description
</button>
<button group="query_get" id="link-query_get-Example"  class="tablinks" 
    onclick="openTab(event, 'query_get', 'Example')">
Example
</button>
</div>

<div group="query_get" id="query_get-Description" class="tabcontent"  style="display: block;" >
Gets a query value for the specified key, it will return the first value found
</div>
<div group="query_get" id="query_get-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/?q=query&b=1");

url.query_get("q"); // "query"
url.query_get("b"); // "1"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> query_getAll </h2>

```rust,ignore
fn query_getAll(url: Url, key: String) -> Array
```

<div>
<div class="tab">
<button group="query_getAll" id="link-query_getAll-Description"  class="tablinks active" 
    onclick="openTab(event, 'query_getAll', 'Description')">
Description
</button>
<button group="query_getAll" id="link-query_getAll-Example"  class="tablinks" 
    onclick="openTab(event, 'query_getAll', 'Example')">
Example
</button>
</div>

<div group="query_getAll" id="query_getAll-Description" class="tabcontent"  style="display: block;" >
Gets a list of values for the specified key

Not available under `no_index`.
</div>
<div group="query_getAll" id="query_getAll-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/?q=query&q=second-query");

url.query_get("q"); // ["query", "second-query"]
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> query_gets </h2>

```rust,ignore
fn query_gets(url: Url, key: String) -> Array
```

<div>
<div class="tab">
<button group="query_gets" id="link-query_gets-Description"  class="tablinks active" 
    onclick="openTab(event, 'query_gets', 'Description')">
Description
</button>
<button group="query_gets" id="link-query_gets-Example"  class="tablinks" 
    onclick="openTab(event, 'query_gets', 'Example')">
Example
</button>
</div>

<div group="query_gets" id="query_gets-Description" class="tabcontent"  style="display: block;" >
Gets a list of values for the specified key

Not available under `no_index`.
</div>
<div group="query_gets" id="query_gets-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/?q=query&q=second-query");

url.query_get("q"); // ["query", "second-query"]
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> query_remove </h2>

```rust,ignore
fn query_remove(url: Url, key: String)
```

<div>
<div class="tab">
<button group="query_remove" id="link-query_remove-Description"  class="tablinks active" 
    onclick="openTab(event, 'query_remove', 'Description')">
Description
</button>
<button group="query_remove" id="link-query_remove-Example"  class="tablinks" 
    onclick="openTab(event, 'query_remove', 'Example')">
Example
</button>
</div>

<div group="query_remove" id="query_remove-Description" class="tabcontent"  style="display: block;" >
Delete a key from the query
</div>
<div group="query_remove" id="query_remove-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/?q=query&b=1");

url.query_delete("q"); // or
url.query_remove("q");

url == "http://test.dev/?b=1"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> query_set </h2>

```rust,ignore
fn query_set(url: Url, key: String, value: String)
```

<div>
<div class="tab">
<button group="query_set" id="link-query_set-Description"  class="tablinks active" 
    onclick="openTab(event, 'query_set', 'Description')">
Description
</button>
<button group="query_set" id="link-query_set-Example"  class="tablinks" 
    onclick="openTab(event, 'query_set', 'Example')">
Example
</button>
</div>

<div group="query_set" id="query_set-Description" class="tabcontent"  style="display: block;" >
Sets a query key
</div>
<div group="query_set" id="query_set-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/?q=query&b=1");

url.query_set("q", "new-query"); // or
url.query_remove("q");

url == "http://test.dev/?q=new-query&b=1"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_debug </h2>

```rust,ignore
fn to_debug(url: Url) -> String
```

<div>
<div class="tab">
<button group="to_debug" id="link-to_debug-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_debug', 'Description')">
Description
</button>
<button group="to_debug" id="link-to_debug-Example"  class="tablinks" 
    onclick="openTab(event, 'to_debug', 'Example')">
Example
</button>
</div>

<div group="to_debug" id="to_debug-Description" class="tabcontent"  style="display: block;" >
Get the absolute url as a string
</div>
<div group="to_debug" id="to_debug-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/path");

url.to_string(); // "http://test.dev/path"
url.to_debug(); // "http://test.dev/path"
```
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> to_string </h2>

```rust,ignore
fn to_string(url: Url) -> String
```

<div>
<div class="tab">
<button group="to_string" id="link-to_string-Description"  class="tablinks active" 
    onclick="openTab(event, 'to_string', 'Description')">
Description
</button>
<button group="to_string" id="link-to_string-Example"  class="tablinks" 
    onclick="openTab(event, 'to_string', 'Example')">
Example
</button>
</div>

<div group="to_string" id="to_string-Description" class="tabcontent"  style="display: block;" >
Get the absolute url as a string
</div>
<div group="to_string" id="to_string-Example" class="tabcontent"  style="display: none;" >

```js
let url = Url("http://test.dev/path");

url.to_string(); // "http://test.dev/path"
url.to_debug(); // "http://test.dev/path"
```
</div>

</div>
</div>
</br>
