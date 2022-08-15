import styles from './index.module.scss';
import BgGradient from '@/assets/password/bg_gradient.png';
import FileIcon from '@/assets/create/file.svg';
import CloseIcon from '@/assets/close.svg';
import { Input, Modal, notification, Radio } from 'antd';
import { useState } from 'react';
import { useHistory } from 'react-router';
import { RouterPath } from '@/router';
import { invoke } from '@tauri-apps/api';
import { ReactComponent as LogoRight } from '@/assets/logo_right.svg';
import { useTranslation } from 'react-i18next';
const Create = () => {
  const { t } = useTranslation();
  const history = useHistory();
  const [visible, setVisible] = useState(false);
  const [type, setType] = useState(0);
  const [key, setKey] = useState('');
  const [name, setName] = useState('');

  const onCreate = async () => {
    /**
     *
     * @type {[]}
     */
    const boxList = (await invoke('box_list')).result;
    const names = boxList.map((value, index) => value.name);
    if (names.indexOf(name) !== -1) {
      notification.open({
        duration: 3,
        message: t('create.modal.box_existed'),
        maxCount: 1,
      });
      return;
    }

    const params = {
      name: name,
      encryptData: type === 0 ? true : false,
      provider: 1,
      accessToken: key,
    };

    await invoke('box_create', { par: params });
    setVisible(false);
    history.replace(RouterPath.box);
  };
  return (
    <div className={styles.createWrap} data-tauri-drag-region>
      <div className={styles.bgWrap}>
        <img src={BgGradient} alt={''} />
      </div>
      <div className={styles.logo}>
        <LogoRight />
      </div>
      <div className={styles.contentWrap}>
        <img src={FileIcon} alt={''} />
        <div className={styles.description}>
          {t('create.create_tips', { msg: 'CipherBox' })}
        </div>
        <div
          onClick={() => {
            setVisible(true);
          }}
          className={styles.confirm}
        >
          {t('create.create')}
        </div>
      </div>
      {visible ? (
        <Modal
          wrapClassName={'createDialogWrap'}
          visible={visible}
          closable={false}
          footer={null}
          width={268}
        >
          <div className={'top'}>
            <div className={'right'}>
              <img src={FileIcon} alt={''} />
              <div>{t('create.modal.create_box')}</div>
            </div>
            <img
              onClick={() => {
                setVisible(false);
              }}
              src={CloseIcon}
              alt={''}
            />
          </div>
          <div className={'content'}>
            <div>
              <div className={'title'}>{t('create.modal.is_security')}</div>
              <Radio.Group
                onChange={(value) => {
                  setType(value.target.value);
                }}
                defaultValue={0}
              >
                <Radio value={0}>{t('create.modal.cipher')}</Radio>
                <Radio style={{ marginLeft: '30px' }} value={1}>
                  {t('create.modal.no_cipher')}
                </Radio>
              </Radio.Group>
            </div>
            <div>
              <div className={'title'}>
                {t('create.modal.select_data_source')}
              </div>
              <Radio.Group defaultValue={1}>
                <Radio value={1}>{t('create.modal.web3.storage')}</Radio>
              </Radio.Group>
              <Input
                placeholder={t('create.modal.enter_token')}
                onChange={(e) => {
                  setKey(e.target.value);
                }}
              />
            </div>
            <div>
              <div className={'title'}>{t('create.modal.name')}</div>
              <Input
                placeholder={t('create.modal.enter_name')}
                onChange={(e) => {
                  setName(e.target.value);
                }}
              />
            </div>
            <div onClick={onCreate} className={'button'}>
              {t('create.modal.create')}
            </div>
          </div>
        </Modal>
      ) : (
        <></>
      )}
    </div>
  );
};

export default Create;
