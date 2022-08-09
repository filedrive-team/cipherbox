import styles from '@/pages/Backup/index.module.scss';
import {
  copyButton,
  copyIcon,
  switchButton,
  switchIcon,
} from '@/styles/home.module.scss';
import { useState } from 'react';
import { useHistory } from 'react-router';
import List from '@/components/List';
import PageControl from '@/components/PageControl';
import { Progress } from 'antd';
import { ReactComponent as DeleteIcon } from '@/assets/backup/delete.svg';
import { ReactComponent as StartIcon } from '@/assets/backup/start.svg';
import { ReactComponent as StopIcon } from '@/assets/backup/stop.svg';
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

const columns = [
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
    title: '状态',
    dataIndex: 'createAt',
    key: 'createAt',
    render: (_, value) => {
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
            percent={value.percent}
            showInfo={false}
            strokeWidth={4}
          ></Progress>
          <div>正在备份中{value.percent}%</div>
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
              console.log('====DeleteIcon====++++=');
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
  const [currentActive, setCurrentActive] = useState(0);

  /**
   * @type [{start:boolean,percent:number,boxId:number,cid:string,createAt:number,  hash:string,id:number,modifyAt:number,name:string, objType:number,originPath:string,path:string,size:number, status:number }]
   */
  const [data, setData] = useState([
    {
      id: 0,
      percent: 40,
      name: '背景总结.png',
      createAt: 0,
      size: 100,
      start: true,
    },
    {
      id: 1,
      percent: 0,
      name: '背景总结.png',
      createAt: 0,
      size: 100,
      start: false,
    },
  ]);

  /**
   * @type [{start:boolean,percent:number,boxId:number,cid:string,createAt:number,  hash:string,id:number,modifyAt:number,name:string, objType:number,originPath:string,path:string,size:number, status:number }]
   */
  const [alreadyData, setAlreadyData] = useState([
    {
      id: 0,
      percent: 40,
      name: '背景总结.png',
      createAt: 0,
      size: 100,
      start: true,
    },
    {
      id: 1,
      percent: 0,
      name: '背景总结.png',
      createAt: 0,
      size: 100,
      start: false,
    },
  ]);

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
              dataSource={data}
              rowKey={(value) => {
                return value.id;
              }}
            />
            <div className={styles.listBottom}>
              {data.length > 10 ? <PageControl total={50} /> : null}
            </div>
          </>
        ) : (
          <>
            <List
              columns={alreadyColumns}
              dataSource={alreadyData}
              rowKey={(value) => {
                return value.id;
              }}
            />
            <div className={styles.listBottom}>
              {data.length > 10 ? <PageControl total={50} /> : null}
            </div>
          </>
        )}
      </div>
    </div>
  );
};

export default Backup;
