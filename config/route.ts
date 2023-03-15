

const routes:any[]=[

    //主页的路由
    {
        path: '/',
        component: '@/layouts/commend',
        routes: [
            { path: '/',redirect:'/commend'},
            {
                path: '/commend',//首页
                component: 'commend' ,
            },
        ],
    }
]
export default routes
