import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import { action, makeObservable, observable, runInAction } from 'mobx';
import _ from 'lodash';

class BackupStore {
  /**
   * @type [{
   * start:boolean,
   * percent:number,
   * boxId:number,
   * cid:string,
   * createAt:number,
   * hash:string,
   * id:number,
   * ,modifyAt:number,
   * name:string,
   * objType:number,
   * originPath:string,
   * path:string,
   * size:number,
   * status:number,
   * originPath:string,
   * total_size:number,
   * finished_size:number
   * }]
   *
   */
  data = [];

  /**
   *@type [{
   * start:boolean,
   * percent:number,
   * boxId:number,
   * cid:string,
   * createAt:number,
   * hash:string,
   * id:number
   * ,modifyAt:number,
   * name:string,
   * objType:number,
   * originPath:string
   * ,path:string,
   * size:number,
   * status:number
   * originPath:string
   * }]
   */
  alreadyData = [];

  constructor() {
    makeObservable(this, {
      data: observable,
      alreadyData: observable,
      fetchData: action,
      fetchAreadyData: action,
      SET_CHANGE_DATA: action,
    });

    listen('task_update', (event) => {
      console.log('+==============event=====00======', event.payload);
      if (event.event === 'task_update') {
        this.SET_CHANGE_DATA(event.payload).then();
      }
    });
  }

  /**
   *
   * @param {{backup:boolean,box_id:number,finished:number,finished_size:number,recover:boolean,task_id:number,total:number,total_size:number}} item
   */
  async SET_CHANGE_DATA(item) {
    let _data = _.clone(this.data);
    _data.map((value, index) => {
      console.log('+=============000=======', value.id, item.task_id);
      if (value.id === item.task_id) {
        console.log('+============111========', item.task_id);

        value.totalSize = item.total_size;
        value.finishedSize = item.finished_size;
      }
      return value;
    });

    runInAction(() => {
      this.data = _data;
    });
  }

  async fetchData() {
    const taskList = await invoke('task_list', {
      status: [0, 1, 6, 9],
    });
    const result = taskList.result;

    console.log('========fetchData=======', result);

    runInAction(() => {
      this.data = result;
    });
  }

  async fetchAreadyData() {
    const taskList = await invoke('task_list', { status: [5] });
    const result = taskList.result;
    console.log('========fetchAreadyData=======', result);

    runInAction(() => {
      this.alreadyData = result;
    });
  }
}

const backupStore = new BackupStore();
export default backupStore;
