import { Disclosure } from "@headlessui/react";
import { Bars3Icon, XMarkIcon } from "@heroicons/react/24/outline";
import styles from "@/styles/nav.module.css";
// import NoSSR from "./NoSSR";

const LOGO = "/zoo-logo-color.svg ";

const navigation = [{ name: "Adventure", href: "/adventure", current: false }];

function classNames(...classes: any) {
  return classes.filter(Boolean).join(" ");
}

export default function InnerNav() {
  // const router = useRouter();
  const navigate = (e: any, url: string) => {
    // e.preventDefault();
    // router.push(url);
  };

  return (
    <Disclosure as="nav" className={styles.nav}>
      {({ open }: { open: boolean }) => (
        <>
          <div className="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
            <div className="relative flex h-16 items-center justify-between">
              <div className="absolute inset-y-0 left-0 flex items-center sm:hidden">
                {/* Mobile menu button*/}
                <Disclosure.Button className="inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white">
                  <span className="sr-only">Open main menu</span>
                  {open ? (
                    <XMarkIcon className="block h-6 w-6" aria-hidden="true" />
                  ) : (
                    <Bars3Icon className="block h-6 w-6" aria-hidden="true" />
                  )}
                </Disclosure.Button>
              </div>
              <div className="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
                <div className="flex flex-shrink-0 items-center">
                  <img
                    className="block h-10 w-auto lg:hidden"
                    height={50}
                    width={50}
                    src={LOGO}
                    alt="Zooliens Home Page"
                    onClick={(e) => navigate(e, "/")}
                  />
                  <img
                    height={50}
                    width={50}
                    className={classNames(
                      styles.logo,
                      "hidden h-10 w-auto lg:block"
                    )}
                    src={LOGO}
                    alt="Zooliens Home Page"
                    onClick={(e) => navigate(e, "/")}
                  />
                </div>
                <div className="hidden sm:ml-6 sm:block">
                  {/* bg-gray-900 text-white  */}
                  <div className="flex space-x-4">
                    {navigation.map((item) => (
                      <a
                        key={item.name}
                        href={item.href}
                        className={classNames(
                          "text-gray-300 hover:bg-gray-700 hover:text-white",
                          "px-3 py-2 rounded-md text-sm font-medium"
                        )}
                        aria-current={item.current ? "page" : undefined}
                      >
                        {item.name}
                      </a>
                    ))}
                  </div>
                </div>
              </div>
            </div>
          </div>

          <Disclosure.Panel className="sm:hidden">
            <div className="space-y-1 px-2 pt-2 pb-3">
              {navigation.map((item) => (
                <Disclosure.Button
                  key={item.name}
                  as="a"
                  href={item.href}
                  className={classNames(
                    item.current
                      ? "bg-gray-900 text-white"
                      : "text-gray-300 hover:bg-gray-700 hover:text-white",
                    "block px-3 py-2 rounded-md text-base font-medium"
                  )}
                  aria-current={item.current ? "page" : undefined}
                >
                  {item.name}
                </Disclosure.Button>
              ))}
            </div>
          </Disclosure.Panel>
        </>
      )}
    </Disclosure>
  );
}
