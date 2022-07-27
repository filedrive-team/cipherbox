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
    title: '备份时间',
    dataIndex: 'createAt',
    key: 'createAt',
  },
];

const Backup = () => {
  const history = useHistory();

  /**
   * @type [{boxId:number,cid:string,createAt:number,  hash:string,id:number,modifyAt:number,name:string, objType:number,originPath:string,path:string,size:number, status:number }]
   */
  const [data, setData] = useState([]);

  return (
    <div>
      <div className={styles.tabWrap}>
        {tabData.map((value, index) => {
          return (
            <div
              key={index}
              className={styles.tabItem}
              style={{ '--bg': value.bg, '--prefix': value.icon }}
            >
              <div className={styles.tabContent}>{value.name}</div>
            </div>
          );
        })}
      </div>
      <div className={styles.listWrap}>
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
      </div>
    </div>
  );
};

export default Backup;
