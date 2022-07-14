import { Suspense } from 'react';
import { BrowserRouter, Switch } from 'react-router-dom';
import Layout from '@/layout';
import { lazy } from 'react';

export class RouterPath {
  static home = '/';
  static test = '/test';
}

const Home = lazy(() => import('@/pages/Home') ?? '');
const Test = lazy(() => import('@/pages/Test') ?? '');

const MRouter = () => {
  return (
    <Suspense fallback={<div></div>}>
      <BrowserRouter>
        <Switch>
          <Layout exact path={RouterPath.home} component={Home}></Layout>
          <Layout exact path={RouterPath.test} component={Test}></Layout>
        </Switch>
      </BrowserRouter>
    </Suspense>
  );
};

export default MRouter;
