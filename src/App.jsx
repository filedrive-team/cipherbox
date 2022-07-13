import './App.css';
import { Provider } from 'mobx-react';
import MRouter from '@/router';
import * as stores from '@/store';
import { useEffect } from 'react';
import '@/styles/main.scss';
function App() {
  useEffect(() => {
    document.addEventListener('contextmenu', async (e) => {
      e.preventDefault();
    });
  }, []);

  return (
    <Provider {...stores}>
      <MRouter />
    </Provider>
  );
}

export default App;
