import { observer } from 'mobx-react';
import SideBar from '@/components/SideBar';
import styles from './index.module.scss';
import classNames from 'classnames';
import { Dropdown, Input, Menu, Table } from 'antd';
import { ReactComponent as SearchIcon } from '@/assets/home/search.svg';
import {
  copyIcon,
  copyButton,
  switchIcon,
  switchButton,
  color435179,
  color3453F4,
} from '@/styles/home.module.scss';
import { useHistory } from 'react-router';
import { RouterPath } from '@/router';
import { invoke } from '@tauri-apps/api';
import { useEffect, useState } from 'react';

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

const Box = () => {
  const [menu, setMenu] = useState();

  async function f() {
    /**
     *
     * @type {[{name:string,accessToken:string,id:number,provider:number}]}
     */
    const boxList = (await invoke('box_list')).result;

    /**
     *
     * @type {{activeBox:{name:string,accessToken:string,id:number,provider:number},hasPasswordSet:boolean,sessionExpired:boolean}}
     */
    const appInfo = (await invoke('app_info')).result;

    let boxItem = boxList?.map((value, index) => {
      return {
        key: index + 1,
        label: (
          <div
            onClick={async () => {
              await invoke('box_set_active', {
                id: value.id,
              });
              f();
            }}
            className={'dropButton'}
            style={{
              '--color':
                value.id === appInfo.activeBox.id ? color3453F4 : color435179,
            }}
          >
            {value.name}
          </div>
        ),
      };
    });

    if (boxItem !== undefined) {
      boxItem.unshift({
        key: 0,
        label: (
          <div
            className={'dropButton'}
            onClick={() => {
              history.push(RouterPath.create);
            }}
          >
            创建盒子
          </div>
        ),
      });
    } else {
      boxItem = [
        {
          key: 0,
          label: (
            <div
              className={'dropButton'}
              onClick={() => {
                history.push(RouterPath.create);
              }}
            >
              创建盒子
            </div>
          ),
        },
      ];
    }
    setMenu(<Menu items={boxItem} />);
  }

  useEffect(() => {
    f();
  }, []);

  const history = useHistory();

  return (
    <div className={styles.homeWrap} onClick={() => {}}>
      <SideBar />
      <div className={classNames(styles.homeBody)}>
        <div className={styles.top} data-tauri-drag-region>
          <Input placeholder={'请输入'} prefix={<SearchIcon />} />
          <div
            onClick={() => {
              history.push('/test');
            }}
            style={{ cursor: 'pointer' }}
          >
            反馈
          </div>
        </div>
        <div className={styles.tabWrap}>
          {tabData.map((value, index) => {
            return index !== 0 ? (
              <Dropdown
                trigger={'click'}
                key={index}
                overlay={menu}
                arrow
                placement="bottom"
              >
                <div
                  className={styles.tabItem}
                  style={{ '--bg': value.bg, '--prefix': value.icon }}
                >
                  {value.name}
                </div>
              </Dropdown>
            ) : (
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

export default observer(Box);
