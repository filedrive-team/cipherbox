class StoragesKey {
  static BoxKey = 'BoxKey';
}

class Storage {
  /**
   *
   * @param  {{type:number,key:string,id:number}}value
   */
  static setBox(value) {
    let boxes = this.getBoxes();
    if (boxes !== null) {
      boxes.push(value);
      localStorage.setItem(StoragesKey.BoxKey, JSON.stringify(boxes));
    } else {
      localStorage.setItem(StoragesKey.BoxKey, JSON.stringify([value]));
    }
  }

  /**
   *
   * @returns {undefined|[{type:number,key:string,id:number}]}
   */
  static getBoxes() {
    return localStorage.getItem(StoragesKey.BoxKey) === null
      ? null
      : JSON.parse(localStorage.getItem(StoragesKey.BoxKey));
  }
}

export default Storage;
