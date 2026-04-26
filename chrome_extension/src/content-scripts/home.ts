import { checkFavoriteStatus } from "./background";

export function main() {
  console.log("processing home...");
  setInterval(() => {
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

          checkFavoriteStatus(uuid!).then((response) => {
            if (response.body.isFavorite) {
              const elem = document.createElement("button"); // as HTMLButtonElement;
              elem.textContent = "See in NFavorites";
              target?.appendChild(elem);
            } else {
              const elem = document.createElement("button"); // as HTMLButtonElement;
              elem.textContent = "Add to NFavorites";
              target?.appendChild(elem);
            }
          })
        }
      }
    }
  }, 100);
}
