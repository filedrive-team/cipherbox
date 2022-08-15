import BgGradient from '@/assets/password/bg_gradient.png';
import codeIcon from '@/assets/password/code.png';
import { ReactComponent as LogoRight } from '@/assets/logo_right.svg';
import styles from './index.module.scss';
import { Input, notification } from 'antd';
import { useHistory } from 'react-router';
import { RouterPath } from '@/router';
import { invoke } from '@tauri-apps/api';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';
const Password = () => {
  const { t } = useTranslation();
  const history = useHistory();
  const [password, setPassword] = useState('');
  const onConfirm = async () => {
    /**
     *
     * @type {{result:{hasPasswordSet:boolean,activeBox:any}}}
     */
    const appInfo = await invoke('app_info');
    if (appInfo.result.hasPasswordSet === true) {
      const password_verify = await invoke('password_verify', {
        password: password,
      });

      if (!password_verify.result) {
        notification.open({
          description: '密码不正确,请重新输入',
          duration: 2,
        });
        setPassword('');
        return;
      }
      if (appInfo.result.activeBox !== null) {
        history.push(RouterPath.box);
      } else {
        history.push(RouterPath.create);
      }
    } else {
      await invoke('password_set', { password: password });
      history.push(RouterPath.create);
    }
  };
  return (
    <div className={styles.passwordWrap} data-tauri-drag-region>
      <div className={styles.bgWrap}>
        <img src={BgGradient} alt={''} />
      </div>
      <div className={styles.logo}>
        <LogoRight />
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
            <Input
              value={password}
              onChange={(e) => {
                setPassword(e.target.value);
              }}
              type={'password'}
              placeholder={t('password.enter_passord')}
            />
            <div onClick={onConfirm} className={styles.confirm}>
              {t('password.confirm')}
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
