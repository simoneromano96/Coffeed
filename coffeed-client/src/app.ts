import { RouterConfiguration, Router } from 'aurelia-router';
import { PLATFORM } from "aurelia-framework"; //! Needed for webpack

export class App {
  router: Router;

  configureRouter(config: RouterConfiguration, router: Router) {
    this.router = router;
    config.title = "Coffeed";
    config.map([
      { route: "", name: "homepage", moduleId: PLATFORM.moduleName("./pages/homepage/homepage") },
      { route: "about", name: "about", moduleId: PLATFORM.moduleName("./pages/about/about") },
      { route: "shop", name: "shop", moduleId: PLATFORM.moduleName("./pages/shop/shop") },
    ]);
  }
}
