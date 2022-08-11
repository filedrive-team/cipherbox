import { invoke } from '@tauri-apps/api';
import { makeObservable, observable, runInAction } from 'mobx';

class BackupStore {
  /**
   * @type [{
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
   * originPath:string,
   * path:string,
   * size:number,
   * status:number
   * originPath:string
   * }]
   *
   */
  data = [];

  /**
   *@type [{start:boolean,
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
    });
  }

  async fetchData() {
    const taskList = await invoke('task_list', {
      status: [0, 1, 3, 4, 5, 6, 7, 8, 9],
    });
    const result = taskList.result;
    runInAction(() => {
      this.data = result;
    });
  }

  async fetchAreadyData() {
    const taskList = await invoke('task_list', { status: [2] });
    const result = taskList.result;
    runInAction(() => {
      this.alreadyData = result;
    });
  }
}

const backupStore = new BackupStore();
export default backupStore;
