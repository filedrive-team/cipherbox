import { Table } from 'antd';
import styles from './index.module.scss';

/**
 *
 * @param {{columns:[],dataSource:[], rowKey?:function}}props
 * @returns {JSX.Element}
 * @constructor
 */
const List = (props) => {
  return (
    <Table
      className={styles.listWrap}
      columns={props.columns}
      dataSource={props.dataSource}
      pagination={false}
      rowKey={(record) => (props?.rowKey ? props?.rowKey(record) : null)}
    />
  );
};

export default List;
