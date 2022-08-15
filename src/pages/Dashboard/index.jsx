import SideBar from '@/components/SideBar';
import styles from './index.module.scss';
import classNames from 'classnames';
import { Input } from 'antd';
import { ReactComponent as SearchIcon } from '@/assets/home/search.svg';

import { RouterPath } from '@/router';
import { Route, Switch } from 'react-router-dom';
import Backup from '@/pages/Backup';
import Box from '@/pages/Box';
import React from 'react';
import { Layout as LT } from 'antd';
const { Sider, Header, Content } = LT;

const Dashboard = () => {
  /**
   * @type [{boxId:number,cid:string,createAt:number,  hash:string,id:number,modifyAt:number,name:string, objType:number,originPath:string,path:string,size:number, status:number }]
   */
  return (
    <div className={styles.homeWrap}>
      <LT>
        <Sider className={styles.layoutSide}>
          <SideBar />
        </Sider>

        <LT className={styles.layoutRight}>
          <Header className={styles.headerWrap}>
            <div className={classNames(styles.homeBody)}>
              <div className={styles.top} data-tauri-drag-region>
                <Input placeholder={'请输入'} prefix={<SearchIcon />} />
                <div onClick={() => {}} style={{ cursor: 'pointer' }}>
                  反馈
                </div>
              </div>
            </div>
          </Header>
          <Content className={styles.contentWrap}>
            <div>
              <Switch>
                <Route
                  exact
                  path={RouterPath.backup}
                  component={Backup}
                ></Route>
                <Route exact path={RouterPath.box} component={Box}></Route>
              </Switch>
            </div>
          </Content>
        </LT>
      </LT>
    </div>
  );
};

export default Dashboard;
