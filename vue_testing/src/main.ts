import Vue from "vue";

import { BootstrapVue, BootstrapVueIcons } from "bootstrap-vue";
import "bootstrap/dist/css/bootstrap.css";
import "bootstrap-vue/dist/bootstrap-vue.css";

import App from "./App.vue";
import router from "./router";

import { createProvider } from "./vue-apollo";
import store from "./store";

Vue.use(BootstrapVue);
Vue.use(BootstrapVueIcons);

Vue.config.productionTip = false;

new Vue({
  router,
  apolloProvider: createProvider(),
  store,
  render: (h) => h(App),
}).$mount("#app");
