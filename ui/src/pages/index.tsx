import profileStyles from "@/styles/profile.module.css";
import Zoomlie from "@/components/Zoomlie";
import Farm from "@/components/Farm";
import { useState } from "react";

const navigation = [
  { name: "Zoomlies", current: false },
  { name: "Farm", current: false },
];

function classNames(...classes: any) {
  return classes.filter(Boolean).join(" ");
}

export default function Profile() {
  const [isZoomlies, setIsZoomlies] = useState(true);

  return (
    <>
      <div className={profileStyles.nav}>
        {navigation.map((item) => (
          <div
            key={item.name}
            className={classNames(
              item.current
                ? "bg-gray-900 text-white"
                : "text-gray-300 hover:bg-gray-700 hover:text-white",
              "block px-3 py-2 rounded-md text-base font-medium"
            )}
            aria-current={item.current ? "page" : undefined}
            onClick={() => {
              if (item.name === "Zoomlies" && !isZoomlies) {
                setIsZoomlies(true);
              } else if (item.name === "Farm" && isZoomlies) {
                setIsZoomlies(false);
              }
            }}
          >
            {item.name}
          </div>
        ))}
      </div>
      {isZoomlies ? <Zoomlie /> : <Farm />}
    </>
  );
}
