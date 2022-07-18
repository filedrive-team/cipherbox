import styles from './index.module.scss';
import classNames from 'classnames';
import { appWindow } from '@tauri-apps/api/window';
import { observer } from 'mobx-react';
import { useEffect } from 'react';
import {
  colorFF6056,
  colorFEBD2D,
  color27CA42,
} from '@/styles/index.module.scss';
import { barClose, barHide, barMax, barMin } from '@/styles/bar.module.scss';

const data = [
  {
    callback: async () => {
      await appWindow.close();
    },
    color: colorFF6056,
    max: barClose,
    min: barClose,
  },
  {
    callback: async () => {
      await appWindow.minimize();
    },
    color: colorFEBD2D,
    max: barHide,
    min: barHide,
  },
  {
    callback: async () => {
      const value = await appWindow.isFullscreen();
      value ? await appWindow.unmaximize() : await appWindow.toggleMaximize();
    },
    color: color27CA42,
    max: barMax,
    min: barMin,
  },
];

const Bar = () => {
  useEffect(() => {}, []);
  return (
    <div className={classNames(styles.barWrap)} data-tauri-drag-region>
      {data.map((value, index) => {
        return (
          <div
            className={classNames(styles.dot)}
            style={{ '--color': value.color, '--img': value.max }}
            onClick={value.callback}
            key={index}
          />
        );
      })}
    </div>
  );
};

export default observer(Bar);
