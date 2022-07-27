import { Route } from 'react-router-dom';
import Bar from '@/components/Bar';

const Layout = (props) => {
  const { component: Com, ...rest } = props;
  return (
    <Route
      {...rest}
      render={(props) => {
        return (
          <div>
            <Bar />
            <Com {...props} />
          </div>
        );
      }}
    />
  );
};

export default Layout;
