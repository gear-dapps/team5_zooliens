import Image from "next/image";
import styles from "@/styles/zoomlie.module.css";
import { useState } from "react";

const HERO = "/blue-hero.svg";

interface Property {
  name: string;
  value: number;
}

interface Zoomlie {
  id: string;
  name: string;
  level: number;
  properties: Property[];
}

export function Zoomlie({ zoomlie }: { zoomlie: Zoomlie }) {
  return (
    <div className={styles.zoomlie}>
      <h1>{zoomlie.name}</h1>
      <Image height={500} width={300} src={HERO} alt="Hero" />
      <div className={styles.properties}>
        {zoomlie.properties.map((p) => (
          <div key={p.name}>
            {p.name}: {p.value}
          </div>
        ))}
      </div>
    </div>
  );
}

export default function Zoomlies() {
  const list: Zoomlie[] = [
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
          <div
            key={item.id}
            className={`flex flex-row p-2`}
            onClick={() => setZoomlie(item)}
          >
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
