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
import taskStore from '@/store/modules/task';
import { observer } from 'mobx-react';
import BigNumber from 'bignumber.js';
import prettyBytes from 'pretty-bytes';
import dayjs from 'dayjs';
import { invoke } from '@tauri-apps/api/tauri';
import { ask } from '@tauri-apps/api/dialog';
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

const Backup = () => {
  const columns = [
    {
      title: 'Path',
      dataIndex: '_originPath',
      key: '_originPath',
      width: 50,
      render: (_, value) => {
        return (
          <Tooltip
            trigger={'click'}
            title={value.taskType === 0 ? value.originPath : value.targetPath}
          >
            <div className={classNames(styles.path)}>
              {value.taskType === 0 ? value.originPath : value.targetPath}
            </div>
          </Tooltip>
        );
      },
    },
    {
      title: 'Size',
      dataIndex: 'totalSize',
      key: 'totalSize',
      align: 'center',
      render: (value) => <div>{value === 0 ? '-' : prettyBytes(value)}</div>,
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
              onClick={async () => {
                const askRes = await ask(
                  'Are you sure want to cancel this task?',
                  {
                    type: 'warning',
                  },
                );

                if (askRes) {
                  await invoke('task_cancel', {
                    id: value.id,
                  });
                  taskStore.SET_TASK_CANCLE(value.id);
                }
              }}
            />
            {value.paused ? (
              <StartIcon
                onClick={async () => {
                  await invoke('task_resume', {
                    id: value.id,
                  });
                  taskStore.SET_TASK_PAUSE(false, value.id);
                }}
              />
            ) : (
              <StopIcon
                onClick={async () => {
                  await invoke('task_pause', {
                    id: value.id,
                  });
                  taskStore.SET_TASK_PAUSE(true, value.id);
                }}
              />
            )}
          </div>
        );
      },
    },
  ];

  const alreadyColumns = [
    {
      title: 'Path',
      dataIndex: '_originPath',
      key: '_originPath',
      width: 100,
      render: (_, value) => {
        return (
          <Tooltip
            trigger={'click'}
            title={value.taskType === 0 ? value.originPath : value.targetPath}
          >
            <div className={classNames(styles.path)}>
              {value.taskType === 0 ? value.originPath : value.targetPath}
            </div>
          </Tooltip>
        );
      },
    },
    {
      title: 'Size',
      dataIndex: 'totalSize',
      key: 'totalSize',
      align: 'center',
      render: (value) => <div>{value === 0 ? '-' : prettyBytes(value)}</div>,
    },
    {
      title: '备份时间',
      dataIndex: 'createAt',
      key: 'createAt',
      render: (value) => (
        <div>{dayjs(new Date(value)).format('YYYY/MM/DD HH:mm')}</div>
      ),
    },
    {
      title: '状态',
      dataIndex: 'operation',
      key: 'operation',
      width: '160',
      align: 'right',
      render: (_, value) => {
        return (
          <div className={styles.operationWrap}>
            {value.taskType === 0 ? '已备份' : '已恢复'}
          </div>
        );
      },
    },
  ];

  const [currentActive, setCurrentActive] = useState(0);

  async function task() {
    taskStore.fetchAreadyData();
    taskStore.fetchData();
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
              dataSource={taskStore.taskData}
              rowKey={(value) => {
                return value.id;
              }}
            />
            <div className={styles.listBottom}>
              {taskStore.data.length > 10 ? <PageControl total={50} /> : null}
            </div>
          </>
        ) : (
          <>
            <List
              columns={alreadyColumns}
              dataSource={taskStore.alreadyData}
              rowKey={(value) => {
                return value.id;
              }}
            />
            <div className={styles.listBottom}>
              {taskStore.alreadyData.length > 10 ? (
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
