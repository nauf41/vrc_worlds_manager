import { checkFavoriteStatus } from "./background";

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
    } catch (e) {
      console.error(e);
      setTimeout(fn, 100);
    }
  }

  setTimeout(fn, 100);
}
