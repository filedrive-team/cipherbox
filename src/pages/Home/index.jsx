import { observer } from 'mobx-react';
import SideBar from '@/components/SideBar';
import styles from './index.module.scss';
const Home = () => {
  return (
    <div className={styles.homeWrap} onClick={() => {}}>
      <SideBar />
      <div></div>
    </div>
  );
};

export default observer(Home);
