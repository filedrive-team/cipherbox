import './index.scss';
import {
  BrowserRouter,
  Redirect,
  Route,
  Switch,
  withRouter,
} from 'react-router-dom';
import Create from '@/pages/create';
import Box from '@/pages/Box';
import Test from '@/pages/Test';
import Password from '@/pages/Password';
import { TransitionGroup, CSSTransition } from 'react-transition-group';
import React from 'react';
import Layout from '@/layout';

export class RouterPath {
  static password = '/password';
  static box = '/box';
  static create = '/create';
  static test = '/test';
}

const DEFAULT_SCENE_CONFIG = {};

const RouterConfig = [
  {
    path: RouterPath.password,
    component: Password,
  },
  {
    path: RouterPath.box,
    component: Box,
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

const getSceneConfig = (location) => {
  const matchedRoute = RouterConfig.find((config) =>
    new RegExp(`^${config.path}$`).test(location.pathname),
  );
  return (matchedRoute && matchedRoute.sceneConfig) || DEFAULT_SCENE_CONFIG;
};

let oldLocation = null;
const Routes = withRouter(({ location, history }) => {
  let classNames = '';
  if (history.action === 'PUSH') {
    classNames = 'forward-' + getSceneConfig(location).enter;
  } else if (history.action === 'POP' && oldLocation) {
    classNames = 'back-' + getSceneConfig(oldLocation).exit;
  }
  oldLocation = location;
  return (
    <TransitionGroup
      className={'router-wrapper'}
      childFactory={(child) => React.cloneElement(child, { classNames })}
    >
      <CSSTransition timeout={500} key={location.pathname}>
        <Switch location={location}>
          {RouterConfig.map((config, index) => {
            return <Layout exact key={index} {...config} />;
          })}
          <Route exact path={'/'}>
            <Redirect to={RouterPath.password} />
          </Route>
        </Switch>
      </CSSTransition>
    </TransitionGroup>
  );
});

const MRouter = () => {
  return (
    <BrowserRouter>
      <Routes />
    </BrowserRouter>
  );
};

export default MRouter;
