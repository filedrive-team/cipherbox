import { Suspense } from 'react';
import { BrowserRouter, Switch } from 'react-router-dom';
import Layout from '@/layout';
import { lazy } from 'react';

class RouterPath {
  static home = '/';
}

const Home = lazy(() => import('@/pages/Home') ?? '');

const MRouter = () => {
  return (
    <Suspense fallback={<div></div>}>
      <BrowserRouter>
        <Switch>
          <Layout exact path={RouterPath.home} component={Home}></Layout>
        </Switch>
      </BrowserRouter>
    </Suspense>
  );
};

export default MRouter;
