import styles from './index.module.scss';
import BgGradient from '@/assets/password/bg_gradient.png';
import FileIcon from '@/assets/create/file.svg';
import CloseIcon from '@/assets/close.svg';
import { Input, Modal, Radio } from 'antd';
import { useState } from 'react';
import { useHistory } from 'react-router';
import { RouterPath } from '@/router';

const Create = () => {
  const history = useHistory();
  const [visible, setVisible] = useState(false);
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
              <Radio.Group defaultValue={1}>
                <Radio value={1}>加密</Radio>
                <Radio style={{ marginLeft: '30px' }} value={2}>
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
              <Input />
            </div>
            <div
              onClick={() => {
                setVisible(false);
                history.push(RouterPath.box);
              }}
              className={'button'}
            >
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
