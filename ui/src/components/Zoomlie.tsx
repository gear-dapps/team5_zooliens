import { useState } from "react";
import { ReactComponent as Zoolie } from "assets/images/zoolie.svg";

interface Property {
  name: string;
  value: number;
}

interface IZoomlie {
  id: string;
  name: string;
  level: number;
  properties: Property[];
}

export function Zoomlie({ zoomlie }: { zoomlie: IZoomlie }) {
  return (
    <div className="text-center">
      <h1>{zoomlie.name}</h1>
      <Zoolie width="20rem" />
      <div>
        {zoomlie.properties.map((p) => (
          <div key={p.name}>
            {p.name}: {p.value}
          </div>
        ))}
      </div>
    </div>
  );
}

export function Zoolies() {
  const list: IZoomlie[] = [
    {
      id: "1",
      name: "ROOGEDY-1",
      level: 12,
      properties: [
        {
          name: "ATTACK",
          value: 84,
        },
        {
          name: "DEFENSE",
          value: 999,
        },
        {
          name: "STAMINA",
          value: 80,
        },
        {
          name: "INTELLIGENCE",
          value: 30,
        },
      ],
    },
    {
      id: "2",
      name: "ROOGEDY-2",
      level: 11,
      properties: [
        {
          name: "ATTACK",
          value: 84,
        },
        {
          name: "DEFENSE",
          value: 78,
        },
        {
          name: "STAMINA",
          value: 80,
        },
        {
          name: "INTELLIGENCE",
          value: 30,
        },
      ],
    },
    {
      id: "3",
      name: "ROOGEDY-3",
      level: 8,
      properties: [
        {
          name: "ATTACK",
          value: 112,
        },
        {
          name: "DEFENSE",
          value: 78,
        },
        {
          name: "STAMINA",
          value: 80,
        },
        {
          name: "INTELLIGENCE",
          value: 30,
        },
      ],
    },
  ];
  const [zoomlie, setZoomlie] = useState(list[0]);

  return (
    <div className="flex flex-row">
      <div className="w-4/12 bg-blue-300">
        {list.map((item) => (
          <div key={item.id} className="flex flex-row p-2">
            <div>
              {item.name} {item.level}
            </div>
          </div>
        ))}
      </div>
      <Zoomlie zoomlie={zoomlie} />
      <div className="w-4/12 h-full">
        <div>Inventory</div>
        <div className="grid grid-cols-3 gap-4">
          <div className="col-span-1">01</div>
          <div className="col-span-1">02</div>
          <div className="col-span-1">03</div>
          <div className="col-span-1">04</div>
          <div className="col-span-1">05</div>
          <div className="col-span-1">06</div>
        </div>
      </div>
    </div>
  );
}
