import { bindable, computedFrom } from "aurelia-framework"

export class CoffeeLoader {
  @bindable loadingPercentage: number
  @bindable name: string

  @computedFrom("loadingPercentage")
  get getJarCss() {
    return `transform: translateY(${50 - this.loadingPercentage / 2}%)`
  }

  @computedFrom("name")
  get getClipPathUrl() {
    return `url(#${name})`
  }
}
