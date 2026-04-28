import { main as home_main } from "./content-scripts/home";
import { main as launch_main } from "./content-scripts/launch";
import { main as world_main } from "./content-scripts/world";

const nowPath = location.pathname.replace(/\/+$/, "") || "/";
console.log("[NWM] content-script started", { href: location.href, pathname: nowPath });

if (nowPath.startsWith("/home/launch")) {
  console.log()
  launch_main();
} else if (nowPath.startsWith("/home/world/")) {
  world_main();
} else if (nowPath === "/home") {
  home_main();
} else {
  console.log("[NWM] route not handled", nowPath);
}
