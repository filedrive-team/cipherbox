import BgGradient from '@/assets/password/bg_gradient.png';
import codeIcon from '@/assets/password/code.png';
import styles from './index.module.scss';
import { Input } from 'antd';
import { useHistory } from 'react-router';
import { RouterPath } from '@/router';

const Password = () => {
  const history = useHistory();
  const onConfirm = () => {
    history.push(RouterPath.create);
  };
  return (
    <div className={styles.passwordWrap}>
      <div className={styles.bgWrap}>
        <img src={BgGradient} alt={''} />
      </div>
      <div className={styles.contentWrap}>
        <div className={styles.leftWrap}>
          <div className={styles.titleWrap}>
            <div
              dangerouslySetInnerHTML={{
                __html:
                  'welocome to'.toUpperCase() +
                  ` <span>{</span>` +
                  ' cipherbox '.toUpperCase() +
                  `<span>}</span>`,
              }}
              className={styles.title}
            ></div>
            <div className={styles.subTitle}>
              Backup your private data to Filecoin
            </div>
          </div>
          <div className={styles.bottom}>
            <Input type={'password'} placeholder={'输入密钥'} />
            <div onClick={onConfirm} className={styles.confirm}>
              确认
            </div>
          </div>
        </div>
        <div className={styles.rightWrap}>
          <img src={codeIcon} alt={''} />
        </div>
      </div>
    </div>
  );
};

export default Password;
