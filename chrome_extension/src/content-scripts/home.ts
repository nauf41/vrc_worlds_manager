import { checkFavoriteStatus, setRegistered, updateCache } from "./background";

export function main() {
  console.log("processing home...");
  const fn = async () => {
    const worlds = document.getElementsByClassName("locations")[0]?.children;
    if (worlds) {
      for (const world of worlds) {
        const target = world
          ?.children[0] // css-1brgsnm
          ?.children[0] // flex-grow-1
          ?.children[1] // align-items-start
          ?.children[2] // algin-self-end
          ?.lastChild; // css-1ecms3y

        if (!["See in NFavorites", "Add to NFavorites"].includes(target?.lastChild?.textContent?.trim() ?? "")) {
          const uuid = (world
            ?.children[0] // css-1brgsnm
            ?.children[0] // flex-grow-1
            ?.children[1] // align-items-start
            ?.children[0] as HTMLLinkElement // a
          )?.href?.match(/worldId=(.+?)&/)?.[1];

          if (!uuid) continue; // not found, skip and try again later

          console.log("Checking favorite status for world", { uuid, world });
          const response = await checkFavoriteStatus(uuid!);
          if (response.body.isFavorite) {
            const elem = document.createElement("button"); // as HTMLButtonElement;
            elem.textContent = "See in NFavorites";
            target?.appendChild(elem);
          } else {
            const elem = document.createElement("button"); // as HTMLButtonElement;
            elem.textContent = "Add to NFavorites";
            elem.addEventListener("click", (e) => {
              setRegistered(
                true,
                {
                  uuid: uuid!,
                }
              ).then(() => {
                elem.remove();
              })
            });
            target?.appendChild(elem);
          }

          updateCache(
            {
              uuid: uuid!,
            },
            {
              description: world
                ?.children[0] // css-1brgsnm
                ?.children[0] // flex-grow-1
                ?.children[1] // align-items-start
                ?.children[1] // mt-2
                ?.textContent ?? null,
              title: world
                ?.children[0] // css-1brgsnm
                ?.children[0] // flex-grow-1
                ?.children[1] // align-items-start
                ?.children[0] // a
                ?.textContent ?? null,
              visits: null,
              favorites: null,
              capacity: null,
              published_at: null,
              does_support_windows: null,
              does_support_android: null,
              does_support_ios: null,
            }
          );
        }
      }
    }
    setTimeout(fn, 1000);
  };

  setTimeout(fn, 1000);
}
