# user

```Namespace: global/user```

<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> birthday </h2>

```rust,ignore
fn birthday() -> Option<DateTime<Utc>>
```

<div>
<div class="tab">
<button group="birthday" id="link-birthday-Description"  class="tablinks active" 
    onclick="openTab(event, 'birthday', 'Description')">
Description
</button>
<button group="birthday" id="link-birthday-Returns"  class="tablinks" 
    onclick="openTab(event, 'birthday', 'Returns')">
Returns
</button>
</div>

<div group="birthday" id="birthday-Description" class="tabcontent"  style="display: block;" >
Gets the user's birthday as a Unix timestamp
</div>
<div group="birthday" id="birthday-Returns" class="tabcontent"  style="display: none;" >
The birthday timestamp
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> id </h2>

```rust,ignore
fn id() -> Option<String>
```

<div>
<div class="tab">
<button group="id" id="link-id-Description"  class="tablinks active" 
    onclick="openTab(event, 'id', 'Description')">
Description
</button>
<button group="id" id="link-id-Returns"  class="tablinks" 
    onclick="openTab(event, 'id', 'Returns')">
Returns
</button>
</div>

<div group="id" id="id-Description" class="tabcontent"  style="display: block;" >
Gets the user's ID
</div>
<div group="id" id="id-Returns" class="tabcontent"  style="display: none;" >
The user's ID, or an empty string if not available
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> language </h2>

```rust,ignore
fn language() -> String
```

<div>
<div class="tab">
<button group="language" id="link-language-Description"  class="tablinks active" 
    onclick="openTab(event, 'language', 'Description')">
Description
</button>
<button group="language" id="link-language-Returns"  class="tablinks" 
    onclick="openTab(event, 'language', 'Returns')">
Returns
</button>
</div>

<div group="language" id="language-Description" class="tabcontent"  style="display: block;" >
Gets the user's language preference
</div>
<div group="language" id="language-Returns" class="tabcontent"  style="display: none;" >
The language code (e.g., "en"), defaults to "en" if not set
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> location </h2>

```rust,ignore
fn location() -> Option<Location>
```

<div>
<div class="tab">
<button group="location" id="link-location-Description"  class="tablinks active" 
    onclick="openTab(event, 'location', 'Description')">
Description
</button>
<button group="location" id="link-location-Returns"  class="tablinks" 
    onclick="openTab(event, 'location', 'Returns')">
Returns
</button>
</div>

<div group="location" id="location-Description" class="tabcontent"  style="display: block;" >
Gets the user's location
</div>
<div group="location" id="location-Returns" class="tabcontent"  style="display: none;" >
A map with 'city' and 'country' fields, or () if not set
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> name </h2>

```rust,ignore
fn name() -> String
```

<div>
<div class="tab">
<button group="name" id="link-name-Description"  class="tablinks active" 
    onclick="openTab(event, 'name', 'Description')">
Description
</button>
<button group="name" id="link-name-Returns"  class="tablinks" 
    onclick="openTab(event, 'name', 'Returns')">
Returns
</button>
</div>

<div group="name" id="name-Description" class="tabcontent"  style="display: block;" >
Gets the user's name
</div>
<div group="name" id="name-Returns" class="tabcontent"  style="display: none;" >
The user's name, or an empty string if not available
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> nickname </h2>

```rust,ignore
fn nickname() -> Option<String>
```

<div>
<div class="tab">
<button group="nickname" id="link-nickname-Description"  class="tablinks active" 
    onclick="openTab(event, 'nickname', 'Description')">
Description
</button>
<button group="nickname" id="link-nickname-Returns"  class="tablinks" 
    onclick="openTab(event, 'nickname', 'Returns')">
Returns
</button>
</div>

<div group="nickname" id="nickname-Description" class="tabcontent"  style="display: block;" >
Gets the user's nickname
</div>
<div group="nickname" id="nickname-Returns" class="tabcontent"  style="display: none;" >
The user's nickname, or () if not set
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> quiet_hours </h2>

```rust,ignore
fn quiet_hours() -> Option<QuietHours>
```

<div>
<div class="tab">
<button group="quiet_hours" id="link-quiet_hours-Description"  class="tablinks active" 
    onclick="openTab(event, 'quiet_hours', 'Description')">
Description
</button>
<button group="quiet_hours" id="link-quiet_hours-Returns"  class="tablinks" 
    onclick="openTab(event, 'quiet_hours', 'Returns')">
Returns
</button>
</div>

<div group="quiet_hours" id="quiet_hours-Description" class="tabcontent"  style="display: block;" >
Gets the user's quiet hours
</div>
<div group="quiet_hours" id="quiet_hours-Returns" class="tabcontent"  style="display: none;" >
A map with 'start' and 'end' fields, or () if not set
</div>

</div>
</div>
</br>
<div style='box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2); padding: 15px; border-radius: 5px; border: 1px solid var(--theme-hover)'>
    <h2 class="func-name"> <code>fn</code> voice_profile_id </h2>

```rust,ignore
fn voice_profile_id() -> Option<String>
```

<div>
<div class="tab">
<button group="voice_profile_id" id="link-voice_profile_id-Description"  class="tablinks active" 
    onclick="openTab(event, 'voice_profile_id', 'Description')">
Description
</button>
<button group="voice_profile_id" id="link-voice_profile_id-Returns"  class="tablinks" 
    onclick="openTab(event, 'voice_profile_id', 'Returns')">
Returns
</button>
</div>

<div group="voice_profile_id" id="voice_profile_id-Description" class="tabcontent"  style="display: block;" >
Gets the user's voice profile ID
</div>
<div group="voice_profile_id" id="voice_profile_id-Returns" class="tabcontent"  style="display: none;" >
The voice profile ID, or () if not set
</div>

</div>
</div>
</br>
