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
import { open } from '@tauri-apps/api/dialog';

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
    dataIndex: 'createAt',
    key: 'createAt',
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

    console.log('==appInfo==', appInfo);

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

  async function f1() {
    /**
     * @type {
     * {activeBox:{name:string,accessToken:string,id:number
     * ,provider:number},
     * hasPasswordSet:boolean,sessionExpired:boolean}}
     */
    const appInfo = (await invoke('app_info')).result;
    /**
     *
     * @type {[{boxId:number,cid:string,createAt:number,
     * hash:string,id:number,modifyAt:number,name:string,
     * objType:number,originPath:string,path:string,size:number,
     * status:number
     * }]}
     */
    const response = (
      await invoke('box_obj_list', {
        boxId: appInfo.activeBox.id,
        lastId: 0,
      })
    ).result;
    setData(response);
  }

  useEffect(() => {
    f();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    f1();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const history = useHistory();
  /**
   * @type [{boxId:number,cid:string,createAt:number,  hash:string,id:number,modifyAt:number,name:string, objType:number,originPath:string,path:string,size:number, status:number }]
   */
  const [data, setData] = useState();

  return (
    <div className={styles.homeWrap}>
      <SideBar />
      <div className={classNames(styles.homeBody)}>
        <div className={styles.top} data-tauri-drag-region>
          <Input placeholder={'请输入'} prefix={<SearchIcon />} />
          <div onClick={() => {}} style={{ cursor: 'pointer' }}>
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
                onClick={async () => {
                  const path = await open();

                  /**
                   * @type {
                   * {activeBox:{name:string,accessToken:string,id:number
                   * ,provider:number},
                   * hasPasswordSet:boolean,sessionExpired:boolean}}
                   */
                  const appInfo = (await invoke('app_info')).result;

                  const response = await invoke('backup', {
                    boxId: appInfo.activeBox.id,
                    targets: [path],
                  });

                  /**
                   *
                   * @type {[{boxId:number,cid:string,createAt:number,
                   * hash:string,id:number,modifyAt:number,name:string,
                   * objType:number,originPath:string,path:string,size:number,
                   * status:number
                   * }]}
                   */
                  const response1 = (
                    await invoke('box_obj_list', {
                      boxId: appInfo.activeBox.id,
                      lastId: 0,
                    })
                  ).result;
                  setData(response1);
                }}
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
