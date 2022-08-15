import styles from './index.module.scss';
import classNames from 'classnames';
import OperationLightIcon from '@/assets/home/operation_light.png';
import OperationIcon from '@/assets/home/operation_default.png';
import ProcessLightIcon from '@/assets/home/process_light.png';
import ProcessIcon from '@/assets/home/process_default.png';
import { ReactComponent as Logo } from '@/assets/home/logo.svg';
import { observer } from 'mobx-react';
import boxStore from '@/store/modules/box';
import { RouterPath } from '@/router';
import { useHistory } from 'react-router-dom';
import { useTranslation } from 'react-i18next';

const SideBar = () => {
  const { t } = useTranslation();

  const history = useHistory();

  /**
   *
   * @type {[{component:React.Component, componentLight:React.Component, title: string}]}
   */
  const data = [
    {
      componentLight: OperationLightIcon,
      component: OperationIcon,
      title: t('side_bar.dashboard'),
      url: RouterPath.box,
    },
    {
      componentLight: ProcessLightIcon,
      component: ProcessIcon,
      title: t('side_bar.task'),
      url: RouterPath.backup,
    },
  ];

  return (
    <div className={classNames(styles.sideWrap)}>
      <div className={styles.logo}>
        <Logo />
      </div>
      <div className={styles.itemWrap}>
        {data.map((value, index) => {
          return (
            <div
              key={index}
              className={styles.item}
              onClick={() => {
                boxStore.SET_TAB_ACTIVE(index);
                switch (index) {
                  case 0:
                    history.push(RouterPath.box);
                    break;
                  case 1:
                    history.push(RouterPath.backup);
                    break;
                }
              }}
            >
              <img
                src={
                  index === boxStore.tabActive
                    ? value.componentLight
                    : value.component
                }
                alt={''}
              />
              {<div>{value.title}</div>}
            </div>
          );
        })}
      </div>
    </div>
  );
};

export default observer(SideBar);
