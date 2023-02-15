import { Route, Routes } from "react-router-dom";
import { Home } from "./home";
import { Adventure } from "./adventure";
import { Submit } from "./submit";

const routes = [
  { path: "/", Page: Home },
  { path: "/adventure", Page: Adventure },
  { path: "/submit", Page: Submit },
];

function Routing() {
  const getRoutes = () =>
    routes.map(({ path, Page }) => (
      <Route key={path} path={path} element={<Page />} />
    ));

  return <Routes>{getRoutes()}</Routes>;
}

export { Routing };
