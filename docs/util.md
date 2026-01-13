# util

```Namespace: global/util```

<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> assert </h2>

```rust,ignore
fn assert(condition: bool, msg: String)
```

<div>
<div class="tab">
<button group="assert" id="link-assert-Description"  class="tablinks active" 
    onclick="openTab(event, 'assert', 'Description')">
Description
</button>
<button group="assert" id="link-assert-Arguments"  class="tablinks" 
    onclick="openTab(event, 'assert', 'Arguments')">
Arguments
</button>
<button group="assert" id="link-assert-Note"  class="tablinks" 
    onclick="openTab(event, 'assert', 'Note')">
Note
</button>
</div>

<div group="assert" id="assert-Description" class="tabcontent"  style="display: block;" >
Asserts a condition, throwing an error if the condition is true
</div>
<div group="assert" id="assert-Arguments" class="tabcontent"  style="display: none;" >
* `condition` - If true, an error will be thrown
* `msg` - The error message to use
</div>
<div group="assert" id="assert-Note" class="tabcontent"  style="display: none;" >
This has inverted logic compared to typical assertions - it errors when true
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> cmd </h2>

```rust,ignore
fn cmd(command: String) -> int
```

<div>
<div class="tab">
<button group="cmd" id="link-cmd-Description"  class="tablinks active" 
    onclick="openTab(event, 'cmd', 'Description')">
Description
</button>
<button group="cmd" id="link-cmd-Arguments"  class="tablinks" 
    onclick="openTab(event, 'cmd', 'Arguments')">
Arguments
</button>
<button group="cmd" id="link-cmd-Returns"  class="tablinks" 
    onclick="openTab(event, 'cmd', 'Returns')">
Returns
</button>
<button group="cmd" id="link-cmd-Note"  class="tablinks" 
    onclick="openTab(event, 'cmd', 'Note')">
Note
</button>
</div>

<div group="cmd" id="cmd-Description" class="tabcontent"  style="display: block;" >
Executes a shell command and returns the exit code
</div>
<div group="cmd" id="cmd-Arguments" class="tabcontent"  style="display: none;" >
* `command` - The command to execute
</div>
<div group="cmd" id="cmd-Returns" class="tabcontent"  style="display: none;" >
The exit code of the command, or -1 if the code couldn't be determined
</div>
<div group="cmd" id="cmd-Note" class="tabcontent"  style="display: none;" >
Uses "cmd /C" on Windows, "sh -c" on Unix-like systems
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> env </h2>

```rust,ignore
fn env(name: String, default: String) -> String
```

<div>
<div class="tab">
<button group="env" id="link-env-Description"  class="tablinks active" 
    onclick="openTab(event, 'env', 'Description')">
Description
</button>
<button group="env" id="link-env-Arguments"  class="tablinks" 
    onclick="openTab(event, 'env', 'Arguments')">
Arguments
</button>
<button group="env" id="link-env-Returns"  class="tablinks" 
    onclick="openTab(event, 'env', 'Returns')">
Returns
</button>
</div>

<div group="env" id="env-Description" class="tabcontent"  style="display: block;" >
Gets an environment variable with a default fallback
</div>
<div group="env" id="env-Arguments" class="tabcontent"  style="display: none;" >
* `name` - The name of the environment variable
* `default` - The default value to return if the variable is not set
</div>
<div group="env" id="env-Returns" class="tabcontent"  style="display: none;" >
The environment variable value, or the default if not found
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> os </h2>

```rust,ignore
fn os() -> String
```

<div>
<div class="tab">
<button group="os" id="link-os-Description"  class="tablinks active" 
    onclick="openTab(event, 'os', 'Description')">
Description
</button>
<button group="os" id="link-os-Returns"  class="tablinks" 
    onclick="openTab(event, 'os', 'Returns')">
Returns
</button>
</div>

<div group="os" id="os-Description" class="tabcontent"  style="display: block;" >
Gets the current operating system name
</div>
<div group="os" id="os-Returns" class="tabcontent"  style="display: none;" >
The OS name (e.g., "linux", "windows", "macos")
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> uuid </h2>

```rust,ignore
fn uuid() -> String
```

<div>
<div class="tab">
<button group="uuid" id="link-uuid-Description"  class="tablinks active" 
    onclick="openTab(event, 'uuid', 'Description')">
Description
</button>
<button group="uuid" id="link-uuid-Returns"  class="tablinks" 
    onclick="openTab(event, 'uuid', 'Returns')">
Returns
</button>
</div>

<div group="uuid" id="uuid-Description" class="tabcontent"  style="display: block;" >
Generates a new UUID v4
</div>
<div group="uuid" id="uuid-Returns" class="tabcontent"  style="display: none;" >
A UUID string in the format "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
</div>

</div>
</div>
</br>
