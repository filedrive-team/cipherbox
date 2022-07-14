import { observer } from 'mobx-react';
import SideBar from '@/components/SideBar';
import styles from './index.module.scss';
import classNames from 'classnames';
import { Input, Table } from 'antd';
import { ReactComponent as SearchIcon } from '@/assets/home/search.svg';
import {
  copyIcon,
  copyButton,
  switchIcon,
  switchButton,
} from '@/styles/home.module.scss';

const tabData = [
  {
    icon: copyIcon,
    bg: copyButton,
    name: '备份',
  },
  {
    icon: switchIcon,
    bg: switchButton,
    name: '切换',
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
    dataIndex: 'time',
    key: 'time',
  },
];

const data = [
  {
    key: '1',
    name: 'John Brown',
    size: 32,
    time: '2022-07-14',
  },
  {
    key: '2',
    name: 'Jim Green',
    size: 32,
    time: '2022-07-14',
  },
  {
    key: '3',
    name: 'Joe Black',
    size: 32,
    time: '2022-07-14',
  },
];

const Home = () => {
  return (
    <div className={styles.homeWrap} onClick={() => {}}>
      <SideBar />
      <div className={classNames(styles.homeBody)}>
        <div className={styles.top} data-tauri-drag-region>
          <Input placeholder={'请输入'} prefix={<SearchIcon />} />
          <div>反馈</div>
        </div>
        <div className={styles.tabWrap}>
          {tabData.map((value, index) => {
            return (
              <div
                key={index}
                className={styles.tabItem}
                style={{ '--bg': value.bg, '--prefix': value.icon }}
              >
                {value.name}
              </div>
            );
          })}
        </div>
        <div className={styles.listWrap}>
          <Table
            columns={columns}
            dataSource={data}
            rowKey={(record) => record.key}
          />
        </div>
      </div>
    </div>
  );
};

export default observer(Home);
