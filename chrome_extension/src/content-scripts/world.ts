import { checkFavoriteStatus, updateCache } from "./background";

export function main() {
  console.log("processing launch...");

  const fn = async () => {
    try {
      const target = document.querySelector(".mt-3.mt-sm-0.css-br1a89.e1264afg10")!;
      const uuid = location.href.match(/wrld_[0-9a-f_-]+/)![0];

      if (uuid && !["See in NFavorites", "Add to NFavorites"].includes(target?.lastChild?.textContent?.trim() ?? "")) {
        console.log("Checking favorite status for world", { uuid });
        const response = await checkFavoriteStatus(uuid!);
        console.log("Favorite status response:", response);
        if (response.body.isFavorite) {
          const elem = document.createElement("button"); // as HTMLButtonElement;
          elem.textContent = "See in NFavorites";
          target.appendChild(elem);
        } else {
          const elem = document.createElement("button"); // as HTMLButtonElement;
          elem.textContent = "Add to NFavorites";
          target.appendChild(elem);
        }
      }

      // on success, update cache
      const platforms = document.querySelector("div.tw-flex.tw-flex-row.tw-gap-1");
      let win = false, android = false, ios = false;
      for (const pf of (platforms as HTMLDivElement)?.children || []) {
        const platform = pf as HTMLDivElement;
        if (platform.title.includes("Windows")) win = true;
        if (platform.title.includes("Android")) android = true;
        if (platform.title.includes("iOS")) ios = true;
      }

      updateCache(
        {
          uuid: location.href.match(/wrld_[0-9a-f_-]+/)![0],
        },
        {
          description: document.querySelector(`div[title="World Description"]`)?.textContent ?? null,
          title: document.querySelector(`h2.tw-overflow-hidden.tw-overflow-ellipsis.tw-line-clamp-2.tw-hyphens-auto.tw-w-full`)?.textContent ?? null,
          visits: parseInt(document.querySelector(`div[aria-label="Visits"]`)?.textContent.replace(",", "") ?? "0"),
          favorites: parseInt(document.querySelector(`div[aria-label="Favorites"]`)?.textContent.replace(",", "") ?? "0"),
          capacity: parseInt(document.querySelector(`div[aria-label="Capacity"]`)?.textContent.replace(",", "") ?? "0"),
          published_at: new Date(document.querySelector(`div[aria-label="Publish Date"]`)?.textContent ?? "").getTime() || null,
          does_support_windows: win,
          does_support_android: android,
          does_support_ios: ios,
        }
      );
    } catch (e) {
      console.error(e);
      setTimeout(fn, 100);
    }
  }
  setTimeout(fn, 100);

  // update cache
  setTimeout(() => {
  }, 5000);
}
