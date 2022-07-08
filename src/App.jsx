import { useState } from 'react'
import logo from './logo.svg'
import './App.css'
import { open, message } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api';

function App() {

  return (
    <div className="App">
      <header className="App-header">
        
        <p>
          <button type="button" onClick={async () => {
            const selected = await open({
              
            });
            if (Array.isArray(selected)) {
              // user selected multiple files
              message(selected.join())
            } else if (selected === null) {
              // user cancelled the selection
              message("nothing selected")
            } else {
              // user selected a single file
              //invoke("backup", {path: selected})
              // try {
              //  await invoke("encrypt_file", {path: selected})
              // } catch(err) {
              //   message(err)
              // }
              try {
               await invoke("encrypt_file", {path: selected})
              } catch(err) {
                message(err)
              }
              
            }
          }}>
            select files to encrypt
          </button>
        </p>
        <p>
          <button type="button" onClick={async () => {
            const selected = await open({
              
            });
            if (Array.isArray(selected)) {
              // user selected multiple files
              message(selected.join())
            } else if (selected === null) {
              // user cancelled the selection
              message("nothing selected")
            } else {
              // user selected a single file
              //invoke("backup", {path: selected})
              // try {
              //  await invoke("encrypt_file", {path: selected})
              // } catch(err) {
              //   message(err)
              // }
              try {
               await invoke("decrypt_file", {path: selected})
              } catch(err) {
                message(err)
              }
              
            }
          }}>
            select files to decrypt
          </button>
        </p>
        <p>
          <a
            className="App-link"
            href="https://reactjs.org"
            target="_blank"
            rel="noopener noreferrer"
          >
            Learn React
          </a>
          {' | '}
          <a
            className="App-link"
            href="https://vitejs.dev/guide/features.html"
            target="_blank"
            rel="noopener noreferrer"
          >
            Vite Docs
          </a>
        </p>
      </header>
    </div>
  )
}

export default App
