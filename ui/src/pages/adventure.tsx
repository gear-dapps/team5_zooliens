function Adventure() {
  const list = [
    { name: "Nikky", level: 10 },
    { name: "Tuma", level: 11 },
  ];

  return (
    <div className="w-full bg-gray-700">
      {list.map((item) => (
        <div key={item.name} className="flex flex-row p-2 justify-around hover">
          <div>{item.name}</div>
          <div>{item.level}</div>
          <div>Fight</div>
        </div>
      ))}
    </div>
  );
}

export { Adventure };
