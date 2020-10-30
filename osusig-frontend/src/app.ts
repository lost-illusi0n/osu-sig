import { bindable } from 'aurelia-framework';

export class App {
  @bindable name: string = '';
  @bindable color: string = '#2a2226';

  @bindable url: string = '';

  nameChanged(newVal, oldVal) {
    this.url = "http://localhost:3030/sig?name="+encodeURIComponent(newVal)+"&color="+encodeURIComponent(this.color)
  }

  colorChanged(newVal, oldVal) {
    this.url = "http://localhost:3030/sig?name="+encodeURIComponent(this.name)+"&color="+encodeURIComponent(newVal)
  }
}
