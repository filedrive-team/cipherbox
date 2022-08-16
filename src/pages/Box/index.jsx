import { observer } from 'mobx-react';
import styles from './index.module.scss';
import classNames from 'classnames';
import { Dropdown, Menu } from 'antd';
import {
  copyIcon,
  copyButton,
  switchIcon,
  switchButton,
  color435179,
  color3453F4,
} from '@/styles/home.module.scss';
import { useHistory } from 'react-router';
import { invoke } from '@tauri-apps/api';
import { useEffect, useState } from 'react';
import { open } from '@tauri-apps/api/dialog';
import { RouterPath } from '@/router';
import { ReactComponent as DownLoadIcon } from '@/assets/box/download.svg';
import { ReactComponent as OpenIcon } from '@/assets/box/open.svg';
import { shell } from '@tauri-apps/api';
import PageControl from '@/components/PageControl';
import List from '@/components/List';
import prettyBytes from 'pretty-bytes';
import dayjs from 'dayjs';
import taskStore from '@/store/modules/task';
import { useTranslation } from 'react-i18next';
const Box = () => {
  const { t } = useTranslation();

  const tabData = [
    {
      icon: copyIcon,
      bg: copyButton,
      name: t('box.backup'),
    },
    {
      icon: switchIcon,
      bg: switchButton,
      name: t('box.switch'),
    },
  ];
  const columns = [
    {
      title: t('box.file_name'),
      dataIndex: 'name',
      key: 'name',
      render: (text) => <div>{text}</div>,
    },
    {
      title: t('box.file_size'),
      dataIndex: 'size',
      key: 'size',
      render: (value) => <div>{prettyBytes(value)}</div>,
    },
    {
      title: t('box.backup_time'),
      dataIndex: 'createAt',
      key: 'createAt',
      render: (value) => (
        <div>{dayjs(new Date(value)).format('YYYY/MM/DD HH:mm')}</div>
      ),
    },
    {
      title: t('box.operation'),
      dataIndex: 'operate',
      key: 'operate',
      render: (_, value) => {
        if (value.exists) {
          return (
            <div
              onClick={async () => {
                await new shell.Command('show-in-finder', [
                  '-R',
                  value.originPath,
                ]).spawn();
              }}
            >
              <OpenIcon />
            </div>
          );
        }
        return (
          <div
            onClick={async () => {
              const path = await open({
                directory: true,
              });

              _ = await invoke('recover', {
                boxId: value.boxId,
                targetDir: path,
                objIds: [value.id],
              });
            }}
          >
            <DownLoadIcon />
          </div>
        );
      },
    },
  ];
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
            {t('box.create')}
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
              {t('box.create')}
            </div>
          ),
        },
      ];
    }
    setMenu(<Menu items={boxItem} />);
  }

  useEffect(() => {
    f();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    taskStore.fetchBoxData();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const history = useHistory();
  /**
   * @type [{exists:boolean,boxId:number,cid:string,createAt:number,  hash:string,id:number,modifyAt:number,name:string, objType:number,originPath:string,path:string,size:number, status:number }]
   */

  return (
    <div className={styles.homeWrap}>
      <div className={classNames(styles.homeBody)}>
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
                  <div className={styles.tabContent}>{value.name}</div>
                </div>
              </Dropdown>
            ) : (
              <div
                key={index}
                className={styles.tabItem}
                style={{ '--bg': value.bg, '--prefix': value.icon }}
                onClick={async () => {
                  const path = await open({
                    multiple: true,
                  });

                  /**
                   * @type {
                   * {activeBox:{name:string,accessToken:string,id:number
                   * ,provider:number},
                   * hasPasswordSet:boolean,sessionExpired:boolean}}
                   */
                  const appInfo = (await invoke('app_info')).result;
                  await invoke('backup', {
                    boxId: appInfo.activeBox.id,
                    targets: path,
                  });
                }}
              >
                <div className={styles.tabContent}>{value.name}</div>
              </div>
            );
          })}
        </div>
        <div className={styles.listWrap}>
          <List
            columns={columns}
            dataSource={taskStore.boxData}
            rowKey={(value) => {
              return value.id;
            }}
          />
          <div className={styles.listBottom}>
            {taskStore.boxData.length > 10 ? <PageControl total={50} /> : null}
          </div>
        </div>
      </div>
    </div>
  );
};

export default observer(Box);
