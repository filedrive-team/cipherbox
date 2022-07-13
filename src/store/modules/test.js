import { action, computed, makeObservable, observable } from 'mobx';

class TestStore {
  count = 0;

  constructor() {
    makeObservable(this, {
      count: observable,
      doubleCount: computed,
      add: action,
    });
  }

  get doubleCount() {
    return this.count * 2;
  }

  async add() {
    this.count++;
    return new Promise((resolve) => {
      setTimeout(() => {
        resolve('');
      }, 5000);
    });
  }
}

const testStore = new TestStore();

export default testStore;
