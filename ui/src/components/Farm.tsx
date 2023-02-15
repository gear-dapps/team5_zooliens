import FarmPNG from "../assets/images/farm.png";

function Farm() {
  const buildings = [
    { name: "Field", value: "100" },
    { name: "Fence", value: "10" },
    { name: "Cab", value: "20" },
    { name: "Barn", value: "150" },
    { name: "Cellar", value: "50" },
  ];

  return (
    <div className="flex flex-row">
      <div className="w-4/12 bg-blue-300">
        {buildings.map((item) => (
          <div key={item.name} className="flex flex-row p-2 hover">
            <div>
              {item.name} +{item.value}
            </div>
          </div>
        ))}
      </div>
      <div className="bg-gray-500 w-full">
        <img src={FarmPNG} alt="farm" />
      </div>
    </div>
  );
}

export { Farm };
