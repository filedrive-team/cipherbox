import styles from '@/pages/Backup/index.module.scss';
import classNames from 'classnames';
import {
  copyButton,
  copyIcon,
  switchButton,
  switchIcon,
} from '@/styles/home.module.scss';
import { useState } from 'react';
import List from '@/components/List';
import PageControl from '@/components/PageControl';
import { Progress, Tooltip } from 'antd';
import { ReactComponent as DeleteIcon } from '@/assets/backup/delete.svg';
import { ReactComponent as StartIcon } from '@/assets/backup/start.svg';
import { ReactComponent as StopIcon } from '@/assets/backup/stop.svg';
import { useEffect } from 'react';
import backupStore from '@/store/modules/backup';
import { observer } from 'mobx-react';
import BigNumber from 'bignumber.js';
const tabData = [
  {
    icon: copyIcon,
    bg: copyButton,
    name: '备份中',
  },
  {
    icon: switchIcon,
    bg: switchButton,
    name: '已备份',
  },
];

const alreadyColumns = [
  {
    title: '文件名',
    dataIndex: 'name',
    key: 'name',
    render: (text) => <div>{text}</div>,
  },
  {
    title: '文件大小',
    dataIndex: 'size',
    key: 'size',
  },
  {
    title: '备份时间',
    dataIndex: 'createAt',
    key: 'createAt',
  },
  {
    title: '状态',
    dataIndex: 'operation',
    key: 'operation',
    width: '160',
    align: 'right',
    render: (_, value) => {
      return <div className={styles.operationWrap}>已备份</div>;
    },
  },
];

const Backup = () => {
  const columns = [
    {
      title: 'Path',
      dataIndex: 'originPath',
      key: 'originPath',
      width: 50,
      render: (text) => {
        return (
          <Tooltip trigger={'click'} title={text}>
            <div className={classNames(styles.path)}>{text}</div>
          </Tooltip>
        );
      },
    },
    {
      title: 'Size',
      dataIndex: 'size',
      key: 'size',
      align: 'center',
      render: (text) => <div>{'-'}</div>,
    },
    {
      title: '进度',
      dataIndex: 'createAt',
      key: 'createAt',
      render: (_, value) => {
        let x = new BigNumber(value.finishedSize);
        let y = new BigNumber(value.totalSize);
        let p = 0;
        if (y.eq(0) === false) {
          p = x.dividedBy(y).times(100).toFormat(2);
        }

        return (
          <div className={styles.progressWrap}>
            <Progress
              strokeColor={{
                '0%': '#32fbff',
                '30%': '#336AFA',
                '100%': '#b199ff',
                direction: '90deg',
              }}
              trailColor={'#F7F7F7'}
              percent={p}
              showInfo={false}
              strokeWidth={4}
            ></Progress>
            <div>正在备份中{parseFloat(p)}%</div>
          </div>
        );
      },
    },
    {
      title: '操作',
      dataIndex: 'operation',
      key: 'operation',
      width: '160',
      align: 'right',
      render: (_, value) => {
        return (
          <div className={styles.operationWrap}>
            <DeleteIcon
              onClick={() => {
                console.log('====DeleteIcon====++++=', backupStore.data);
              }}
            />

            {value.start ? (
              <StopIcon
                onClick={() => {
                  console.log('====StopIcon====++++=');
                }}
              />
            ) : (
              <StartIcon
                onClick={() => {
                  console.log('====StartIcon====++++=');
                }}
              />
            )}
          </div>
        );
      },
    },
  ];

  const [currentActive, setCurrentActive] = useState(0);

  async function task() {
    backupStore.fetchAreadyData();
    backupStore.fetchData();
  }

  useEffect(() => {
    task();
  }, []);

  return (
    <div>
      <div className={styles.tabWrap}>
        {tabData.map((value, index) => {
          return (
            <div
              key={index}
              className={styles.tabItem}
              style={{ '--bg': value.bg, '--prefix': value.icon }}
              onClick={() => {
                setCurrentActive(index);
              }}
            >
              <div className={styles.tabContent}>{value.name}</div>
            </div>
          );
        })}
      </div>
      <div className={styles.listWrap}>
        {currentActive === 0 ? (
          <>
            <List
              columns={columns}
              dataSource={backupStore.data}
              rowKey={(value) => {
                return value.id;
              }}
            />
            <div className={styles.listBottom}>
              {backupStore.data.length > 10 ? <PageControl total={50} /> : null}
            </div>
          </>
        ) : (
          <>
            <List
              columns={alreadyColumns}
              dataSource={backupStore.alreadyData}
              rowKey={(value) => {
                return value.id;
              }}
            />
            <div className={styles.listBottom}>
              {backupStore.alreadyData.length > 10 ? (
                <PageControl total={50} />
              ) : null}
            </div>
          </>
        )}
      </div>
    </div>
  );
};

export default observer(Backup);
