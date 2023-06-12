import {createRouter,createWebHashHistory} from 'vue-router';
import VersionList from '@/views/version-list/index.vue';
import SystemManagement from '@/views/system-management/index.vue';
import PageConfiguration from '@/views/system-management/page-configuration/index.vue';
import RelationPermission from '@/views/system-management/relation-permission/index.vue';

const routes = [
    { 
        path: '/',
        name:'version-list',
        component: VersionList 
    },
    { 
        path: '/version-list',
        name:'version-list',
        component: VersionList 
    },
    { 
        path: '/system-management', 
        name: 'system-management',
        component: SystemManagement,
        children:[
            {
                path:'/page-configuration',
                name:'page-configuration',
                component: PageConfiguration,
            },
            {
                path:'/relation-permission',
                name:'relation-permission',
                component: RelationPermission,
            },
        ]  
    },
    {
        path: '/:catchAll(.*)',
        redirect: '/version-list'
    }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router;