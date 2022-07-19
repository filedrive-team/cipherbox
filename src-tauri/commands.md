## Tauri commands 
How to communicate with javascript running in webview

#### app_info
Get basic info of cipherbox
```js
let appInfo = await invoke('app_info', null);
// {
//      "error": "",
//      "result": {
//          "hasPasswordSet": false,
//          "sessionExpired": false 
//      }
// }
```
rust struct 
```rust,no_run
pub struct AppInfo {
    // indicate whether user has set password or not
    pub has_password_set: bool, 
    // valid session period after password been verified
    // will expire in a centain time, currently not implemented
    pub session_expired: bool,
}
```

### password_set
Set user password
```js
import { invoke } from '@tauri-apps/api'

await invoke('password_set', {password: 'main_password_for_cipherbox'})
```

### password_verify
Verify user password
```js
import { invoke } from '@tauri-apps/api'

await invoke('password_verify', {password: 'main_password_for_cipherbox'})
```

#### box_create
Create box for user
```js
let boxInfo = await invoke('box_create', {
    name: "box01",
    encryptData: true,
    provider: 1,
    accessToken: "token_from_provider::currently_support_web3storage"
});

```
rust struct 
```rust,no_run
// input
pub struct CreateCboxParams {
    pub name: String,
    pub encrypt_data: bool,
    // currently just filled with 1
    pub provider: i32,
    pub access_token: String,
}

// output
pub struct CBox {
    pub id: i32,
    pub name: String,
    // most of time backup should be encrypt unless user intentionly set it false, maybe for public share
    pub encrypt_data: bool,
    // total objects in the box
    pub obj_total: u64,
    // total size of objects in the box
    pub size_total: u64,
    // the key use to do encrypt works
    #[serde(skip_deserializing)]
    pub secret: Vec<u8>,
    // the storage provider, like web3.storage
    pub provider: i32,
    // access token for provider api
    pub access_token: String,
    // the current showing box for user
    pub active: u8,
}
```

#### box_list
Get box list
```js
let boxList = await invoke('box_list', null);

```

### box_set_active
Get box list
```js
let activeBox = await invoke('box_set_active', {id: 1});

```

### backup
Select files to backup
```js
await invoke('back', {box_id: 1, targets: ["/path/to/file"]});

```

#### box_obj_list
Get box list
```js
let boxObjList = await invoke('box_obj_list', {box_id:1, last_id: 0});

```