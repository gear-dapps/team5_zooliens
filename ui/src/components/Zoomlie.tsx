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
    <div className="text-center w-6/12">
      <h1>{zoomlie.name}</h1>
      <div className="flex align-center justify-center">
        <Zoolie width="20rem" />
      </div>
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
      <div className="w-3/12 bg-gray-900 pt-5">
        {list.map((item) => (
          <div // eslint-disable-line jsx-a11y/no-static-element-interactions
            key={item.id}
            className="flex flex-row p-2 justify-around hover"
            onClick={(e) => {
              e.preventDefault();
              setZoomlie(item);
            }}
            onKeyDown={() => {}}
          >
            <div>{item.name}</div>
            <div> {item.level}</div>
          </div>
        ))}
      </div>
      <Zoomlie zoomlie={zoomlie} />
      <div className="w-3/12 bg-gray-900 p-5">
        <div className="text-center pb-5">Inventory</div>
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
