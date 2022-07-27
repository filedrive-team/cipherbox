import { Pagination } from 'antd';
import './index.module.scss';
/**
 *
 * @param {{total:number}}props
 * @returns {JSX.Element}
 * @constructor
 */
const PageControl = (props) => {
  return (
    <>
      <Pagination total={props.total}></Pagination>
    </>
  );
};

export default PageControl;
