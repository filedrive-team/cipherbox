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

const Create = () => {
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
        message: '盒子已存在!',
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
        <div className={styles.description}>创建属于你的Cipherbox</div>
        <div
          onClick={() => {
            setVisible(true);
          }}
          className={styles.confirm}
        >
          创建
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
              <div>创建Cipherbox</div>
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
              <div className={'title'}>box是否加密</div>
              <Radio.Group
                onChange={(value) => {
                  setType(value.target.value);
                }}
                defaultValue={0}
              >
                <Radio value={0}>加密</Radio>
                <Radio style={{ marginLeft: '30px' }} value={1}>
                  不加密
                </Radio>
              </Radio.Group>
            </div>
            <div>
              <div className={'title'}>选择数据源</div>
              <Radio.Group defaultValue={1}>
                <Radio value={1}>Web3.storage</Radio>
              </Radio.Group>
              <Input
                type={'password'}
                placeholder={'输入token'}
                onChange={(e) => {
                  setKey(e.target.value);
                }}
              />
            </div>
            <div>
              <div className={'title'}>Box 名称</div>
              <Input
                placeholder={'输入名称'}
                onChange={(e) => {
                  setName(e.target.value);
                }}
              />
            </div>
            <div onClick={onCreate} className={'button'}>
              创建
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
