import styles from './index.module.scss';
import BgGradient from '@/assets/password/bg_gradient.png';
import FileIcon from '@/assets/create/file.svg';
import CloseIcon from '@/assets/close.svg';
import { Input, Modal, Radio } from 'antd';
import { useState } from 'react';
import { useHistory } from 'react-router';
import Storage from '@/data/storage';
import { RouterPath } from '@/router';
import { confirm } from '@tauri-apps/api/dialog';

const Create = () => {
  const history = useHistory();
  const [visible, setVisible] = useState(false);
  const [type, setType] = useState(0);
  const [key, setKey] = useState('');
  const onCreate = async () => {
    let values = Storage.getBoxes();
    console.log(values);

    if (values !== null) {
      let keys = values?.map((value) => value.key);
      console.log(keys, key);
      if (keys.indexOf(key) !== -1) {
        const confirmed = await confirm('是否创建一个已存在的盒子?', {
          type: 'warning',
        });
        if (confirmed) {
          Storage.setBox({
            type: type,
            key: key,
            id: Date.now(),
          });
          setVisible(false);
          history.push(RouterPath.box);
        }
        return;
      }
      Storage.setBox({
        type: type,
        key: key,
        id: Date.now(),
      });
      setVisible(false);
      history.push(RouterPath.box);
    }
  };
  return (
    <div className={styles.createWrap}>
      <div className={styles.bgWrap}>
        <img src={BgGradient} alt={''} />
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
            </div>
            <div>
              <div className={'title'}>文件名</div>
              <Input
                onChange={(e) => {
                  setKey(e.target.value);
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
