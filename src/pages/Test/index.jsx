const Test = () => {
  return (
    <div
      onClick={() => {
        window.history.back();
      }}
      style={{
        background: 'yellow',
        position: 'absolute',
        width: '100vw',
        height: '100vh',
      }}
    >
      测试
    </div>
  );
};

export default Test;
