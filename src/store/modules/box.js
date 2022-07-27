import { makeObservable, observable, runInAction } from 'mobx';

class BoxStore {
  tabActive = 0;

  constructor() {
    makeObservable(this, {
      tabActive: observable,
    });
  }

  /**
   *
   * @param {number}value
   * @constructor
   */
  SET_TAB_ACTIVE(value) {
    runInAction(() => {
      this.tabActive = value;
    });
  }
}

const boxStore = new BoxStore();
export default boxStore;
