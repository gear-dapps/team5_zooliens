function Adventure() {
  const list = [
    { name: "Nikky", level: 10 },
    { name: "Tuman", level: 7 },
  ];

  return (
    <div className="w-full bg-blue-300">
      {list.map((item) => (
        <div key={item.name} className="flex flex-row p-2">
          <div>
            {item.name} level: {item.level} Fight
          </div>
        </div>
      ))}
    </div>
  );
}

export { Adventure };
