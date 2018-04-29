import Vue from 'vue'
import Router from 'vue-router'
import Vuex from 'vuex'
import VueI18n from 'vue-i18n'
import BootstrapVue from 'bootstrap-vue'
import ElementUI from 'element-ui'

import Application from '@/layouts/application'
import Dashboard from '@/layouts/dashboard'
import nut from './nut'
import forum from './forum'

Vue.use(Router)
Vue.use(Vuex)
Vue.use(VueI18n)
Vue.use(BootstrapVue)
Vue.use(ElementUI)

Vue.component('layout-application', Application)
Vue.component('layout-dashboard', Dashboard)

const plugins = [nut, forum]

export default {
  router: new Router({
    mode: 'history',
    routes: plugins.reduce((ar, it) => ar.concat(it.routes), [])
  })
}
