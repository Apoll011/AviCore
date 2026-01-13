# fs

```Namespace: global/fs```

<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> append </h2>

```rust,ignore
fn append(path: String, content: String)
```

<div>
<div class="tab">
<button group="append" id="link-append-Description"  class="tablinks active" 
    onclick="openTab(event, 'append', 'Description')">
Description
</button>
<button group="append" id="link-append-Arguments"  class="tablinks" 
    onclick="openTab(event, 'append', 'Arguments')">
Arguments
</button>
<button group="append" id="link-append-Returns"  class="tablinks" 
    onclick="openTab(event, 'append', 'Returns')">
Returns
</button>
</div>

<div group="append" id="append-Description" class="tabcontent"  style="display: block;" >
Appends a string to the end of a file
</div>
<div group="append" id="append-Arguments" class="tabcontent"  style="display: none;" >
* `path` - The path to the file to append to
* `content` - The string to append to the file
</div>
<div group="append" id="append-Returns" class="tabcontent"  style="display: none;" >
Nothing
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> basename </h2>

```rust,ignore
fn basename(path: String) -> String
```

<div>
<div class="tab">
<button group="basename" id="link-basename-Description"  class="tablinks active" 
    onclick="openTab(event, 'basename', 'Description')">
Description
</button>
<button group="basename" id="link-basename-Arguments"  class="tablinks" 
    onclick="openTab(event, 'basename', 'Arguments')">
Arguments
</button>
<button group="basename" id="link-basename-Returns"  class="tablinks" 
    onclick="openTab(event, 'basename', 'Returns')">
Returns
</button>
</div>

<div group="basename" id="basename-Description" class="tabcontent"  style="display: block;" >
Gets the last component of a path
</div>
<div group="basename" id="basename-Arguments" class="tabcontent"  style="display: none;" >
* `path` - The path to process
</div>
<div group="basename" id="basename-Returns" class="tabcontent"  style="display: none;" >
The last component of the path
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> copy </h2>

```rust,ignore
fn copy(src: String, dest: String) -> bool
```

<div>
<div class="tab">
<button group="copy" id="link-copy-Description"  class="tablinks active" 
    onclick="openTab(event, 'copy', 'Description')">
Description
</button>
<button group="copy" id="link-copy-Arguments"  class="tablinks" 
    onclick="openTab(event, 'copy', 'Arguments')">
Arguments
</button>
<button group="copy" id="link-copy-Returns"  class="tablinks" 
    onclick="openTab(event, 'copy', 'Returns')">
Returns
</button>
</div>

<div group="copy" id="copy-Description" class="tabcontent"  style="display: block;" >
Copies a file from one path to another
</div>
<div group="copy" id="copy-Arguments" class="tabcontent"  style="display: none;" >
* `src` - The source path
* `dest` - The destination path
</div>
<div group="copy" id="copy-Returns" class="tabcontent"  style="display: none;" >
True if the copy was successful, false otherwise
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> delete </h2>

```rust,ignore
fn delete(path: String) -> bool
```

<div>
<div class="tab">
<button group="delete" id="link-delete-Description"  class="tablinks active" 
    onclick="openTab(event, 'delete', 'Description')">
Description
</button>
<button group="delete" id="link-delete-Arguments"  class="tablinks" 
    onclick="openTab(event, 'delete', 'Arguments')">
Arguments
</button>
<button group="delete" id="link-delete-Returns"  class="tablinks" 
    onclick="openTab(event, 'delete', 'Returns')">
Returns
</button>
</div>

<div group="delete" id="delete-Description" class="tabcontent"  style="display: block;" >
Deletes a file or an empty directory
</div>
<div group="delete" id="delete-Arguments" class="tabcontent"  style="display: none;" >
* `path` - The path to delete
</div>
<div group="delete" id="delete-Returns" class="tabcontent"  style="display: none;" >
True if the deletion was successful, false otherwise
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> dirname </h2>

```rust,ignore
fn dirname(path: String) -> String
```

<div>
<div class="tab">
<button group="dirname" id="link-dirname-Description"  class="tablinks active" 
    onclick="openTab(event, 'dirname', 'Description')">
Description
</button>
<button group="dirname" id="link-dirname-Arguments"  class="tablinks" 
    onclick="openTab(event, 'dirname', 'Arguments')">
Arguments
</button>
<button group="dirname" id="link-dirname-Returns"  class="tablinks" 
    onclick="openTab(event, 'dirname', 'Returns')">
Returns
</button>
</div>

<div group="dirname" id="dirname-Description" class="tabcontent"  style="display: block;" >
Gets the parent directory of a path
</div>
<div group="dirname" id="dirname-Arguments" class="tabcontent"  style="display: none;" >
* `path` - The path to process
</div>
<div group="dirname" id="dirname-Returns" class="tabcontent"  style="display: none;" >
The parent directory of the path
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> exists </h2>

```rust,ignore
fn exists(path: String) -> bool
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
Checks if a path exists
</div>
<div group="exists" id="exists-Arguments" class="tabcontent"  style="display: none;" >
* `path` - The path to check
</div>
<div group="exists" id="exists-Returns" class="tabcontent"  style="display: none;" >
True if the path exists, false otherwise
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> list_files </h2>

```rust,ignore
fn list_files(path: String) -> Vec<String>
```

<div>
<div class="tab">
<button group="list_files" id="link-list_files-Description"  class="tablinks active" 
    onclick="openTab(event, 'list_files', 'Description')">
Description
</button>
<button group="list_files" id="link-list_files-Arguments"  class="tablinks" 
    onclick="openTab(event, 'list_files', 'Arguments')">
Arguments
</button>
<button group="list_files" id="link-list_files-Returns"  class="tablinks" 
    onclick="openTab(event, 'list_files', 'Returns')">
Returns
</button>
</div>

<div group="list_files" id="list_files-Description" class="tabcontent"  style="display: block;" >
Lists the names of files and directories in a given path
</div>
<div group="list_files" id="list_files-Arguments" class="tabcontent"  style="display: none;" >
* `path` - The path to list
</div>
<div group="list_files" id="list_files-Returns" class="tabcontent"  style="display: none;" >
A list of file and directory names
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> mkdir </h2>

```rust,ignore
fn mkdir(path: String) -> bool
```

<div>
<div class="tab">
<button group="mkdir" id="link-mkdir-Description"  class="tablinks active" 
    onclick="openTab(event, 'mkdir', 'Description')">
Description
</button>
<button group="mkdir" id="link-mkdir-Arguments"  class="tablinks" 
    onclick="openTab(event, 'mkdir', 'Arguments')">
Arguments
</button>
<button group="mkdir" id="link-mkdir-Returns"  class="tablinks" 
    onclick="openTab(event, 'mkdir', 'Returns')">
Returns
</button>
</div>

<div group="mkdir" id="mkdir-Description" class="tabcontent"  style="display: block;" >
Creates a directory and all its parent directories if they don't exist
</div>
<div group="mkdir" id="mkdir-Arguments" class="tabcontent"  style="display: none;" >
* `path` - The path of the directory to create
</div>
<div group="mkdir" id="mkdir-Returns" class="tabcontent"  style="display: none;" >
True if the directory was created successfully, false otherwise
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> move </h2>

```rust,ignore
fn move(src: String, dest: String) -> bool
```

<div>
<div class="tab">
<button group="move" id="link-move-Description"  class="tablinks active" 
    onclick="openTab(event, 'move', 'Description')">
Description
</button>
<button group="move" id="link-move-Arguments"  class="tablinks" 
    onclick="openTab(event, 'move', 'Arguments')">
Arguments
</button>
<button group="move" id="link-move-Returns"  class="tablinks" 
    onclick="openTab(event, 'move', 'Returns')">
Returns
</button>
</div>

<div group="move" id="move-Description" class="tabcontent"  style="display: block;" >
Moves or renames a file or directory
</div>
<div group="move" id="move-Arguments" class="tabcontent"  style="display: none;" >
* `src` - The source path
* `dest` - The destination path
</div>
<div group="move" id="move-Returns" class="tabcontent"  style="display: none;" >
True if the move was successful, false otherwise
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> read </h2>

```rust,ignore
fn read(path: String) -> String
```

<div>
<div class="tab">
<button group="read" id="link-read-Description"  class="tablinks active" 
    onclick="openTab(event, 'read', 'Description')">
Description
</button>
<button group="read" id="link-read-Arguments"  class="tablinks" 
    onclick="openTab(event, 'read', 'Arguments')">
Arguments
</button>
<button group="read" id="link-read-Returns"  class="tablinks" 
    onclick="openTab(event, 'read', 'Returns')">
Returns
</button>
</div>

<div group="read" id="read-Description" class="tabcontent"  style="display: block;" >
Reads the entire contents of a file as a string
</div>
<div group="read" id="read-Arguments" class="tabcontent"  style="display: none;" >
* `path` - The path to the file to read
</div>
<div group="read" id="read-Returns" class="tabcontent"  style="display: none;" >
The file contents as a string, or UNIT if the file could not be read
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> write </h2>

```rust,ignore
fn write(path: String, content: String)
```

<div>
<div class="tab">
<button group="write" id="link-write-Description"  class="tablinks active" 
    onclick="openTab(event, 'write', 'Description')">
Description
</button>
<button group="write" id="link-write-Arguments"  class="tablinks" 
    onclick="openTab(event, 'write', 'Arguments')">
Arguments
</button>
<button group="write" id="link-write-Returns"  class="tablinks" 
    onclick="openTab(event, 'write', 'Returns')">
Returns
</button>
</div>

<div group="write" id="write-Description" class="tabcontent"  style="display: block;" >
Writes a string to a file, overwriting its contents
</div>
<div group="write" id="write-Arguments" class="tabcontent"  style="display: none;" >
* `path` - The path to the file to write
* `content` - The string to write to the file
</div>
<div group="write" id="write-Returns" class="tabcontent"  style="display: none;" >
Nothing
</div>

</div>
</div>
</br>
