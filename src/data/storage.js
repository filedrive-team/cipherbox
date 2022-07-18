class StoragesKey {
  static BoxKey = 'BoxKey';
}

class Storage {
  /**
   *
   * @param value
   */
  static setBox(value) {
    localStorage.setItem(
      StoragesKey.BoxKey,
      value !== null ? JSON.stringify(value) : null,
    );
  }

  /**
   *
   * @returns {null|any}
   */
  static getBox() {
    return localStorage.getItem(StoragesKey.BoxKey) === null
      ? null
      : JSON.parse(localStorage.getItem(StoragesKey.BoxKey));
  }
}

export default Storage;
