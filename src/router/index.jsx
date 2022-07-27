import './index.scss';
import { BrowserRouter, Redirect, Route, Switch } from 'react-router-dom';
import Create from '@/pages/create';
import Test from '@/pages/Test';
import Password from '@/pages/Password';
import Dashboard from '@/pages/Dashboard';
import React from 'react';
import Layout from '@/layout';

export class RouterPath {
  static password = '/password';
  static box = '/dashboard/box';
  static create = '/create';
  static test = '/test';
  static dashboard = '/dashboard';
  static operation = '/dashboard/operation';
  static backup = '/dashboard/backup';
}

const RouterConfig = [
  {
    path: RouterPath.password,
    component: Password,
  },
  {
    path: RouterPath.dashboard,
    component: Dashboard,
  },
  {
    path: RouterPath.create,
    component: Create,
  },
  {
    path: '/test',
    component: Test,
    sceneConfig: {
      enter: 'from-right',
      exit: 'to-right',
    },
  },
];

const Routes = () => {
  return (
    <Switch>
      {RouterConfig.map((value, index) => {
        return (
          <Layout
            exact={value.path !== RouterPath.dashboard}
            key={index}
            {...value}
          ></Layout>
        );
      })}
      <Route exact path={'/'}>
        <Redirect to={RouterPath.password} />
      </Route>
    </Switch>
  );
};

const MRouter = () => {
  return (
    <BrowserRouter>
      <Routes />
    </BrowserRouter>
  );
};

export default MRouter;
