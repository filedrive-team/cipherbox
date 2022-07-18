## Tauri commands 
How to communicate with javascript running in webview

#### app_info
Get basic info of cipherbox
```js
let appInfo = await invoke('app_info', null);
// {
//   "has_password_set": false
//   "session_expired": false 
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