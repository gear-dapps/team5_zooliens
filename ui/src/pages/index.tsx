import { Route, Routes } from "react-router-dom";
import { Home } from "./home";
import { Adventure } from "./adventure";
import { Mint } from "./mint";

const routes = [
  { path: "/", Page: Home },
  { path: "/adventure", Page: Adventure },
  { path: "/mint", Page: Mint },
];

function Routing() {
  const getRoutes = () =>
    routes.map(({ path, Page }) => (
      <Route key={path} path={path} element={<Page />} />
    ));

  return <Routes>{getRoutes()}</Routes>;
}

export { Routing };
