import styles from './index.module.scss';
import classNames from 'classnames';
import { ReactComponent as OperationIcon } from '@/assets/home/operation.svg';
const SideBar = () => {
  return (
    <div className={classNames(styles.sideWrap)}>
      <div className={styles.item}>
        <OperationIcon />
        <div>工作台</div>
      </div>
    </div>
  );
};

export default SideBar;
