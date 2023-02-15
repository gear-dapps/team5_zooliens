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
      <div className="w-4/12 bg-gray-700">
        {buildings.map((item) => (
          <div
            key={item.name}
            className="flex flex-row p-5 hover justify-around"
          >
            <div>{item.name}</div>
            <div> +{item.value}</div>
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
