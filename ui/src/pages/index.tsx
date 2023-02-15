import { Route, Routes } from "react-router-dom";
import { Home } from "./home";
import { Adventure } from "./Adventure";

const routes = [
  { path: "/", Page: Home },
  { path: "/adventure", Page: Adventure },
];

function Routing() {
  const getRoutes = () =>
    routes.map(({ path, Page }) => (
      <Route key={path} path={path} element={<Page />} />
    ));

  return <Routes>{getRoutes()}</Routes>;
}

export { Routing };
