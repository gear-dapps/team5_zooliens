/* eslint-disable */
import { useSearchParams } from "react-router-dom";
// import { useApi, useAccount } from "@gear-js/react-hooks";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

// const DEST =
//  "0xdb2d60f773c3f45c70dcae69612f27ca122350198731e5cc5bf59732d8497384";

function Mint() {
  const [searchParams] = useSearchParams();
  const id = searchParams.get("id");
  const key = searchParams.get("key");
  const navigate = useNavigate();
  // const { api } = useApi();
  // const { account } = useAccount();

  const [idValue, setIdValue] = useState(id ? id : "");
  const [keyValue, setKeyValue] = useState(key ? key : "");

  const onChangeId = (value: string) => {
    setIdValue(value);
  };

  const onChangeKey = (value: string) => {
    setKeyValue(value);
  };

  const doMint = async () => {
    const minted = localStorage.getItem("mint");
    if (minted != null) {
      const items = JSON.parse(minted);
      if (items.list.includes(idValue)) {
        alert(`Zoolien ${idValue} has already been minted.`);
        return;
      }
    }

    let newItems = minted == null ? { list: [] } : JSON.parse(minted);
    newItems.list.push(idValue);
    localStorage.setItem("mint", JSON.stringify(newItems));
    navigate("/", {});
  };

  return (
    <>
      <div className="flex min-h-full flex-col justify-center py-12 sm:px-6 lg:px-8">
        <div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
          <div className="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
            <div className="space-y-6">
              <div>
                <label
                  htmlFor="email"
                  className="block text-sm font-medium text-gray-700"
                >
                  ID
                </label>
                <div className="mt-1 text-black">
                  <input
                    id="email"
                    type="text"
                    required
                    className="block w-full appearance-none rounded-md border border-gray-300 px-3 py-2 placeholder-gray-400 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                    value={idValue}
                    onChange={(e) => onChangeId(e.target.value)}
                  />
                </div>
              </div>

              <div>
                <label
                  htmlFor="password"
                  className="block text-sm font-medium text-gray-700"
                >
                  PRIVATE KEY
                </label>
                <div className="mt-1 text-black">
                  <input
                    id="password"
                    name="password"
                    type="password"
                    autoComplete="current-password"
                    required
                    className="block w-full appearance-none rounded-md border border-gray-300 px-3 py-2 placeholder-gray-400 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                    value={keyValue}
                    onChange={(e) => onChangeKey(e.target.value)}
                  />
                </div>
              </div>

              <div>
                <button
                  className="flex w-full justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                  onClick={() => doMint()}
                >
                  MINT
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}

export { Mint };
