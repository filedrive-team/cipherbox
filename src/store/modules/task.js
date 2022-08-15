import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import { action, makeObservable, observable, runInAction } from 'mobx';
import _ from 'lodash';
import { exists } from 'tauri-plugin-fs-extra-api';

class TaskStore {
  /**
   * @type [{
   * paused:boolean,
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
   * total_size:number,
   * finished_size:number
   * targetPath:string,
   * taskType:number
   * }]
   *
   */
  data = [];

  get taskData() {
    return this.data.filter((value) => {
      return value.status !== 7;
    });
  }

  /**
   *@type [{
   * paused:boolean,
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
   * originPath:string,
   * targetPath:string,
   * taskType:number
   * }]
   */
  alreadyData = [];

  /**
   * @type [{exists:boolean,boxId:number,cid:string,createAt:number,
   * hash:string,id:number,modifyAt:number,name:string,
   * objType:number,originPath:string,path:string,size:number,
   * status:number
   * }]
   */
  boxData = [];

  constructor() {
    makeObservable(this, {
      data: observable,
      alreadyData: observable,
      boxData: observable,
      fetchData: action,
      fetchAreadyData: action,
      SET_CHANGE_DATA: action,
      fetchBoxData: action,
    });

    listen('task_update', (event) => {
      if (event.event === 'task_update') {
        if (event.payload.finished === 1) {
          this.fetchAreadyData();
          this.fetchData();
          this.fetchBoxData();
        }
        if (this.data.length === 0) {
          this.fetchData();
        }
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
      if (value.id === item.task_id) {
        value.paused = false;
        value.totalSize = item.total_size;
        value.finishedSize = item.finished_size;
      }
      return value;
    });

    runInAction(() => {
      this.data = _data;
    });
  }

  /**
   *
   * @param {boolean} paused
   * @param {number} id
   */
  async SET_TASK_PAUSE(paused, id) {
    let _data = _.clone(this.data);
    _data.map((value, index) => {
      if (value.id === id) {
        value.paused = paused;
      }
      return value;
    });
    runInAction(() => {
      this.data = _data;
    });
  }

  /**
   *
   * @param {*} id
   */
  async SET_TASK_CANCLE(id) {
    let _data = _.clone(this.data);
    _data.map((value, index) => {
      if (value.id === id) {
        console.log('======================+++');
        value.status = 7;
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
    let result = taskList.result;
    let _data = result.map((value) => {
      if (value.status === 6) {
        value.paused = true;
      }
      return value;
    });
    runInAction(() => {
      this.data = _data;
    });
  }

  async fetchAreadyData() {
    const taskList = await invoke('task_list', { status: [5] });
    const result = taskList.result;

    runInAction(() => {
      this.alreadyData = result;
    });
  }

  async fetchBoxData() {
    /**
     * @type {
     * {activeBox:{name:string,accessToken:string,id:number
     * ,provider:number},
     * hasPasswordSet:boolean,sessionExpired:boolean}}
     */
    const appInfo = (await invoke('app_info')).result;

    /**
     *
     * @type {[{exists:boolean,boxId:number,cid:string,createAt:number,
     * hash:string,id:number,modifyAt:number,name:string,
     * objType:number,originPath:string,path:string,size:number,
     * status:number
     * }]}
     */
    const response = (
      await invoke('box_obj_list', {
        boxId: appInfo.activeBox.id,
        lastId: 0,
      })
    ).result;

    const response_map = await Promise.all(
      response.map(async (value, index) => {
        const _exists = await exists(value.originPath);
        value.exists = _exists;
        return value;
      }),
    );

    console.log('+==================response_map=========', response_map);
    runInAction(() => {
      this.boxData = response_map;
    });
  }
}

const taskStore = new TaskStore();
export default taskStore;
