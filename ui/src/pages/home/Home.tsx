import { Farm } from "components/Farm";
import { useState } from "react";
import { Zoolies } from "../../components/Zoomlie";

const navigation = [
  { name: "Zoolies", current: false },
  { name: "Farm", current: false },
];

function classNames(...classes: any) {
  return classes.filter(Boolean).join(" ");
}

function Home() {
  const [isZoomlies, setIsZoomlies] = useState(true);

  return (
    <>
      <div className="flex flex-row">
        {navigation.map((item) => (
          <div // eslint-disable-line jsx-a11y/no-static-element-interactions
            key={item.name}
            className={classNames(
              item.current
                ? "bg-gray-900 text-white"
                : "text-gray-300 hover:bg-gray-700 hover:text-white",
              "block px-3 py-2 rounded-md text-base font-medium"
            )}
            aria-current={item.current ? "page" : undefined}
            onClick={(e) => {
              e.preventDefault();
              if (item.name === "Zoolies" && !isZoomlies) {
                setIsZoomlies(true);
              } else if (item.name === "Farm" && isZoomlies) {
                setIsZoomlies(false);
              }
            }}
            onKeyDown={() => {}}
          >
            {item.name}
          </div>
        ))}
      </div>
      {isZoomlies ? <Zoolies /> : <Farm />}
    </>
  );
}

export { Home };
